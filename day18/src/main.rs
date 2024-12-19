use std::{
    collections::HashSet,
    hash::Hash,
    io::{stdin, BufRead, BufReader, Read},
    cmp::Reverse,
};

use priority_queue::PriorityQueue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Pretty basic heuristic
    fn distance_from_origin(&self) -> usize {
        self.x as usize + self.y as usize
    }

    fn distance_to(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    fn generate_neighbors(&self, width: i32, height: i32, corrupted: &HashSet<Point>) -> Vec<Point> {
        vec![
            Point { x: self.x, y: self.y - 1 },
            Point { x: self.x, y: self.y + 1 },
            Point { x: self.x - 1, y: self.y },
            Point { x: self.x + 1, y: self.y },
        ].into_iter()
            .filter(|p|
                p.x >= 0 && p.y >= 0
                && p.x < width && p.y < height
                && !corrupted.contains(p))
            .collect()
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (raw_x, raw_y) = value
            .split_once(',')
            .unwrap();
        
        Point {
            x: raw_x.parse::<i32>().expect("Not a valid integer"),
            y: raw_y.parse::<i32>().expect("Not a valid integer"),
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance_from_origin().cmp(&other.distance_from_origin())
    }
}

#[derive(Clone)]
struct Map {
    width: i32,
    height: i32,
    corrupted: HashSet<Point>,
}

impl Map {
    fn new(width: i32, height: i32) -> Self {
        Map {
            width,
            height,
            corrupted: HashSet::new(),
        }
    }

    fn corrupt(&mut self, byte_coords: &[Point]) {
        for coords in byte_coords.into_iter() {
            self.corrupted.insert(coords.clone());
        }
    }

    fn find_exit(&self) -> Option<usize> {
        let origin = Point { x: 0, y: 0 };
        let target = Point { x: self.width - 1, y: self.height - 1 };

        let mut visited: HashSet<Point> = HashSet::new();
        let mut front_scores = PriorityQueue::new();
        front_scores.push(origin.clone(), Reverse((origin.distance_to(&target), 0usize)));
        let mut front_set: HashSet<Point> = HashSet::from_iter(vec![origin.clone()].into_iter());

        while let Some((current, rev_steps)) = front_scores.pop() {
            let (_prio, current_steps) = rev_steps.0;

//            std::thread::sleep(std::time::Duration::from_millis(10));
            front_set.remove(&current);

//            self.print(current_steps, &current, &visited, &front_set);

            let all_neighbors = current.generate_neighbors(self.width, self.height, &self.corrupted);
            visited.insert(current);

            for neighbor in all_neighbors.into_iter() {
                if neighbor == target {
                    return Some(current_steps + 1)
                }

                let next_step = current_steps + 1;

                if !visited.contains(&neighbor) {
                    if front_set.contains(&neighbor) {
                        let it = front_scores.iter_mut();
                        let (_, prio) = it
                            .skip_while(|(p, _)| *p != &neighbor)
                            .next()
                            .unwrap();

                        let (distance, steps) = prio.0;

                        if steps > next_step {
                            *prio = Reverse((distance - steps + next_step, next_step))
                        }
                    } else {
                        let d = neighbor.distance_to(&target);
                        front_set.insert(neighbor.clone());
                        front_scores.push(neighbor, Reverse((d + next_step, next_step)));
                    }
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn print(&self, steps: usize, testing: &Point, visited: &HashSet<Point>, front: &HashSet<Point>) {
        print!("\x1b[H");
        for row in 0..self.height {
            for col in 0..self.width {
                let p = Point { x: col, y: row };
                if self.corrupted.contains(&p) {
                    print!("#");
                } else if &p == testing {
                    print!("X");
                } else if visited.contains(&p) {
                    print!("+");
                } else if front.contains(&p) {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!("");
        }

        println!("\n X at {steps} steps");
    }
}

fn dicotomic_search(initial_map: Map, bytes: Vec<Point>, good: usize, bad: usize) -> Point {
    let mut good = good;
    let mut bad = bad;

    while good < bad {
        let next_attempt = (good + bad) / 2;
        if good == next_attempt {
            break;
        }

        let mut test_map = initial_map.clone();
        test_map.corrupt(&bytes[..next_attempt]);

        if test_map.find_exit().is_none() {
            bad = next_attempt;
        } else {
            good = next_attempt;
        }
    }

    let mut bytes = bytes;

    bytes.remove(bad - 1)
}

fn read_bytes<R>(stream: BufReader<R>) -> Vec<Point>
where 
    R: Read
{
    stream
        .lines()
        .map(|l| Point::from(l.unwrap().trim_end()))
        .collect()
}

fn is_test() -> bool {
    std::env::args().any(|s| s == "--test")
}

fn main() {
    // print!("\x1bc");
    let testing = is_test();
    let limits: (i32, i32) = if testing { (6, 6) } else { (70, 70) };
    let bytes = read_bytes(BufReader::new(stdin()));
    let mut map = Map::new(limits.0 + 1, limits.1 + 1);

    if testing {
        map.corrupt(&bytes[..12]);
    } else {
        map.corrupt(&bytes[..1024]);
    }

    println!("The exit can be reached in {} steps", map.find_exit().unwrap());

    let mut good = 1023;

    loop {
        let next_attempt = (good * 2).max(bytes.len());
        let mut next_map = Map::new(limits.0 + 1, limits.1 + 1);
        next_map.corrupt(&bytes[..next_attempt]);

        if next_map.find_exit().is_none() {
            let new_map = Map::new(limits.0 + 1, limits.1 + 1);
            let needle = dicotomic_search(new_map, bytes, good, next_attempt);

            println!("It seems like the byte that takes the cake is coords: {},{}", needle.x, needle.y);
            break;
        } else {
            good = next_attempt;
        }
    }
}
