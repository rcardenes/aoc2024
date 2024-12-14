use std::io::{stdin, BufRead, BufReader, Read};

const TILES_WIDE: i32 = 101;
const MID_WIDE: i32 = TILES_WIDE / 2;
const TILES_TALL: i32 = 103;
const MID_TALL: i32 = TILES_TALL / 2;

fn coords_from_text(raw: &str) -> (i32, i32) {
    let numbers = raw.split_once('=').unwrap().1;
    let (x, y) = numbers.split_once(',').unwrap();

    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn quadrant(&self) -> Option<u32> {
        match (self.x, self.y) {
            (MID_WIDE, _) | (_, MID_TALL) => None,
            (x, y) if x < MID_WIDE && y < MID_TALL => { Some(0) },
            (x, y) if x < MID_WIDE && y > MID_TALL => { Some(1) },
            (x, y) if x > MID_WIDE && y < MID_TALL => { Some(2) },
            (x, y) if x > MID_WIDE && y > MID_TALL => { Some(3) },
            _ => panic!("This shouldn't happen...")
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
    eprintln!("Safety factor after 100 seconds: {safety_factor}");
}
