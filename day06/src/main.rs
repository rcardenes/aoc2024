use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader, Read}
};
use itertools::{Itertools, Either};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction { Up, Down, Left, Right }

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coords {
    row: i32,
    col: i32,
}

impl Coords {
    fn up(&self) -> Self { Coords { row: self.row - 1, col: self.col, } }
    fn down(&self) -> Self { Coords { row: self.row + 1, col: self.col, } }
    fn left(&self) -> Self { Coords { row: self.row, col: self.col - 1, } }
    fn right(&self) -> Self { Coords { row: self.row, col: self.col + 1, } }

    fn move_to(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GuardMotion {
    coords: Coords,
    direction: Direction,
}

impl GuardMotion {
    fn up(coords: Coords) -> GuardMotion {
        GuardMotion { coords, direction: Direction::Up }
    }
    fn down(coords: Coords) -> GuardMotion {
        GuardMotion { coords, direction: Direction::Down }
    }
    fn left(coords: Coords) -> GuardMotion {
        GuardMotion { coords, direction: Direction::Left }
    }
    fn right(coords: Coords) -> GuardMotion {
        GuardMotion { coords, direction: Direction::Right }
    }

    fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }

    fn forward(&self) -> Coords {
        self.coords.move_to(self.direction)
    }
}

#[derive(Debug)]
struct Map {
    rows: usize,
    cols: usize,
    guard: GuardMotion,
    obstacle_coords: HashSet<Coords>
}

impl Map {
    fn out_of_bounds(&self, coords: &Coords) -> bool {
        coords.row < 0 || coords.row >= (self.rows as i32) || coords.col < 0 || coords.col >= (self.cols as i32)
    }

    fn hit_obstacle(&self, coords: &Coords) -> bool {
        self.obstacle_coords.contains(coords)
    }

    fn print_walk(&self, positions: &HashSet<Coords>) {
        for row in 0..self.rows as i32 {
            for col in 0..self.cols as i32 {
                let coords = Coords { row, col };

                if positions.contains(&coords) {
                    print!("X");
                }
                else if self.obstacle_coords.contains(&coords) {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn find_unique_positions(map: &Map) -> usize {
    let initial = &map.guard;
    let mut current_guard = initial.clone();

    let mut seen = HashSet::new();
    let mut exit_positions = HashSet::new();

    seen.insert(current_guard.coords.clone());
    exit_positions.insert(initial.clone());

    loop {
        let next_coords = current_guard.forward();

        if map.out_of_bounds(&next_coords) {
            break;
        } else if map.hit_obstacle(&next_coords) {
            current_guard.turn_right();
            exit_positions.insert(current_guard.clone());
        } else {
            seen.insert(next_coords.clone());
            current_guard.coords = next_coords;

            if exit_positions.contains(&current_guard) {
                break;
            }
        }
    }

    seen.len()
}

// Brute force. Nasty, but effective
fn find_loop_options(map: &Map) -> usize {
    let mut options = 0usize;
    let initial = &map.guard;

    for row in 0..map.rows as i32 {
        for col in 0..map.cols as i32 {
            let new_obstacle_coords = Coords { row, col };

            if new_obstacle_coords == initial.coords || map.obstacle_coords.contains(&new_obstacle_coords) {
                continue
            }

            let mut current_guard = initial.clone();
            let mut exit_positions = HashSet::new();
            exit_positions.insert(initial.clone());

            loop {
                let next_coords = current_guard.forward();

                if map.out_of_bounds(&next_coords) {
                    break;
                } else if next_coords == new_obstacle_coords || map.hit_obstacle(&next_coords) {
                    exit_positions.insert(current_guard.clone());
                    current_guard.turn_right();
                } else {
                    current_guard.coords = next_coords;

                    if exit_positions.contains(&current_guard) {
                        options += 1;
                        break;
                    }
                }
            }
        }
    }

    options
}

#[derive(Debug)]
enum Object {
    Obstacle(Coords),
    Guard(GuardMotion),
}

fn read_map<R>(stream: BufReader<R>) -> Map
    where R: Read,
{
    let raw_map = stream.lines().into_iter()
        .map(|l| l.unwrap().trim_end().to_string())
        .collect::<Vec<_>>();

    let rows = raw_map.len();
    let cols = raw_map[0].len();
    let (obstacle_coords, mut guard): (Vec<Coords>, Vec<GuardMotion>) = raw_map.into_iter()
        .enumerate()
        .flat_map(|(row, line)|
            line.chars()
                .enumerate()
                .filter_map(|(col, c)| {
                    let coord = Coords { row: row as i32, col: col as i32};
                    match c {
                        '#' => Some(Object::Obstacle(coord)),
                        '^' => Some(Object::Guard(GuardMotion::up(coord))),
                        'v' => Some(Object::Guard(GuardMotion::down(coord))),
                        '<' => Some(Object::Guard(GuardMotion::left(coord))),
                        '>' => Some(Object::Guard(GuardMotion::right(coord))),
                        _ => None
                    }
                })
                .collect::<Vec<_>>()
        )
        .partition_map(|o| match o {
            Object::Obstacle(coords) => Either::Left(coords),
            Object::Guard(guard_motion) => Either::Right(guard_motion),
        });

    let guard = guard.pop().unwrap();

    Map {
        rows,
        cols,
        guard,
        obstacle_coords: HashSet::from_iter(obstacle_coords.into_iter()),
    }
}

fn main() {
    let map = read_map(BufReader::new(stdin()));

    let unique = find_unique_positions(&map);

    println!("Unique positions: {unique}");

    let loop_options = find_loop_options(&map);

    println!("Obstruction positions: {loop_options}");
}
