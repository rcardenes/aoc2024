use std::{collections::HashMap, io::{stdin, BufRead, BufReader, Read}};

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct ScoredPoint {
    coords: Point,
    score: usize
}

impl ScoredPoint {
    fn new(coords: &Point) -> Self {
        ScoredPoint {
            coords: coords.clone(),
            score: 0
        }
    }
}


struct Map {
    rows: Vec<Vec<u8>>,
    bottoms: Vec<Point>,
}

impl Map {
    fn new(rows: Vec<Vec<u8>>) -> Self {
        let bottoms = rows.iter().enumerate()
            .flat_map(|(r, row)|
                row.iter().enumerate()
                    .filter(|(_, &val)| val == 0)
                    .map(|(c, _)| Point { y: r as i32, x: c as i32 })
                    .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();
        
        Map {
            rows,
            bottoms,
        }
    }

    fn find_trailheads(&self) {

        let mut scores: HashMap<Point, usize> = HashMap::new();

        for start in self.bottoms.iter() {
            let mut trail: Vec<ScoredPoint> = vec![];
            let wavefront: Vec<Point> = vec![start.clone()];

            while !wavefront.empty() {
                let current = wavefront.pop().unwrap();

                if scores.contains(
            }
        }
    }
}

fn read_map<R>(stream: BufReader<R>) -> Map
    where R: Read
{
    let rows = stream
        .lines()
        .map(|l|
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Map::new(rows)
}

fn main() {
    let map = read_map(BufReader::new(stdin()));

    map.find_trailheads();
}
