use std::{collections::HashMap, io::{stdin, BufRead, BufReader, Read}};

const TILES_WIDE: i32 = 101;
const MID_WIDE: i32 = TILES_WIDE / 2;
const TILES_TALL: i32 = 103;
const MID_TALL: i32 = TILES_TALL / 2;

fn coords_from_text(raw: &str) -> (i32, i32) {
    let numbers = raw.split_once('=').unwrap().1;
    let (x, y) = numbers.split_once(',').unwrap();

    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn quadrant(&self) -> Option<u32> {
        match (self.x, self.y) {
            (MID_WIDE, _) | (_, MID_TALL) => None,
            (x, y) => {
                if x < MID_WIDE && y < MID_TALL { Some(0) }
                else if x < MID_WIDE && y > MID_TALL { Some(1) }
                else if x > MID_WIDE && y < MID_TALL { Some(2) }
                else { Some(3) }
            }
        }
    }
}

#[derive(Debug)]
struct Robot {
    pos: Point,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn move_by(&self, steps: i32) -> Self {
        let x = (self.pos.x + self.dx * steps).rem_euclid(TILES_WIDE);
        let y = (self.pos.y + self.dy * steps).rem_euclid(TILES_TALL);

        Robot {
            pos: Point { x, y },
            dx: self.dx,
            dy: self.dy,
        }
    }

    fn quadrant(&self) -> Option<u32> {
        self.pos.quadrant()
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (position, velocity) = value.split_once(' ').unwrap();

        let (x, y) = coords_from_text(position);
        let (dx, dy) = coords_from_text(velocity);
        let pos = Point { x, y };

        Robot { pos, dx, dy }
    }
}

fn might_be_tree(robots: &Vec<Robot>) -> bool {
    let mut at_line: HashMap<i32, Vec<i32>> = HashMap::new();

    for r in robots.iter() {
        at_line.entry(r.pos.y)
            .and_modify(|v| v.push(r.pos.x))
            .or_insert(vec![1]);
    }

    for value in at_line.into_values() {
        if value.len() > 20 {
            let mut value = value;
            value.sort();
            let (consec, _) = value[1..]
                .iter()
                .fold((0, value[0]),
                    |(acc, prev_x), x| if (x - prev_x) == 1 { (acc + 1, *x) } else { (acc, *x) } );
            return consec > 15
        }
    }

    false
}

fn read_problem<R>(stream: BufReader<R>) -> Vec<Robot>
where
    R: Read,
{
    stream
        .lines()
        .map(|line| Robot::from(line.unwrap().trim_end()))
        .collect()
}

fn main() {
    let robots = read_problem(BufReader::new(stdin()));

    let mut in_quadrant = [0usize; 4];
    robots.iter()
        .filter_map(|r| r.move_by(100).quadrant())
        .for_each(|q| in_quadrant[q as usize] += 1);

    let safety_factor = in_quadrant.into_iter().fold(1usize, |acc, n| acc * n);
    println!("Safety factor after 100 seconds: {safety_factor}");

    let mut robots = robots;
    for t in 0..10000 {
        if might_be_tree(&robots) {
            println!("Found tree after {t} seconds");
            break;
        }
        robots = robots.into_iter().map(|r| r.move_by(1)).collect();
    }
}
