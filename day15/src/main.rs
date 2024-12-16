use std::{collections::HashSet, io::{stdin, BufRead, BufReader, Read}};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Wrong direction: {value}"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_row_col(row: usize, col: usize) -> Self {
        Point {
            x: col as i32,
            y: row as i32,
        }
    }

    fn step_to(&self, dir: &Direction) -> Point {
        let (dx, dy) = dir.delta();
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn gps_coordinate(&self) -> i32 {
        self.y * 100 + self.x
    }
}

struct Map {
    width: usize,
    height: usize,
    walls: HashSet<Point>,
    boxes: HashSet<Point>,
    robot: Point,
}

impl Map {
    fn follow(&self, instructions: &Vec<Direction>) -> Map {
        let walls = self.walls.clone();
        let mut boxes = self.boxes.clone();
        let mut robot = self.robot.clone();

        for instr in instructions.iter() {
            let mut blocked = true;
            let mut next_step = robot.step_to(&instr);
            let mut boxes_to_move = vec![];
            while !walls.contains(&next_step)  {
                if boxes.contains(&next_step) {
                    boxes_to_move.push(next_step.clone());
                    next_step = next_step.step_to(&instr);
                } else {
                    blocked = false;
                    break;
                }
            }

            if !blocked {
                robot = robot.step_to(&instr);
                while let Some(bx) = boxes_to_move.pop() {
                    boxes.remove(&bx);
                    boxes.insert(bx.step_to(&instr));
                }
            }
        }

        Map {
            width: self.width,
            height: self.height,
            walls,
            boxes,
            robot,
        }
    }

    fn sum_coords(&self) -> i32 {
        self.boxes.iter().map(|b| b.gps_coordinate()).sum::<i32>()
    }

    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let p = Point::from_row_col(row, col);

                if self.boxes.contains(&p) {
                    print!("O");
                }
                else if self.walls.contains(&p) {
                    print!("#");
                }
                else if self.robot == p {
                    print!("@");
                }
                else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

impl From<Vec<String>> for Map {
    fn from(value: Vec<String>) -> Self {
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot: Option<Point> = None;
        let height = value.len();
        let width = value[0].len();

        for (row, line) in value.into_iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match c {
                    '#' => { walls.insert(Point::from_row_col(row, col)); },
                    'O' => { boxes.insert(Point::from_row_col(row, col)); },
                    '@' => robot = Some(Point::from_row_col(row, col)),
                    '.' => {}, // Just ignore it
                    _ => panic!("Shouldn't exist!")
                }
            }
        }

        Map {
            width,
            height,
            walls,
            boxes,
            robot: robot.unwrap()
        }
    }
}

fn read_map<R>(stream: BufReader<R>) -> (Map, Vec<Direction>)
where
    R: Read
{
    let mut lines = stream.lines();

    let map_lines = lines.by_ref()
        .map(|l| l.unwrap().trim_end().to_string())
        .take_while(|l| l.len() > 0)
        .collect::<Vec<_>>();

    let instructions = lines
        .map(|l| l.unwrap().trim_end().to_string())
        .skip_while(|l| l.len() == 0)
        .flat_map(|l| l.chars().map(|c| Direction::from(c)).collect::<Vec<_>>())
        .collect();

    (Map::from(map_lines), instructions)
}

fn main() {
    let (map, instr) = read_map(BufReader::new(stdin()));

    let after = map.follow(&instr);
    let sum_coords = after.sum_coords();

    eprintln!("Sum of box coordinateS: {sum_coords}");
}
