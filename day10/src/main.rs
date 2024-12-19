use std::{collections::HashMap, io::{stdin, BufRead, BufReader, Read}};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn adjacent(&self) -> Vec<Point> {
        vec![
            Point { x: self.x - 1, y: self.y },
            Point { x: self.x + 1, y: self.y },
            Point { x: self.x, y: self.y - 1 },
            Point { x: self.x, y: self.y + 1 },
        ]
    }
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

struct TrailStep<'a> {
    coords: Point,
    prev: Option<&'a TrailStep<'a>>
}

impl<'a> TrailStep<'a> {
    fn new(coords: Point) -> Self {
        TrailStep {
            coords,
            prev: None,
        }
    }

    fn with_parent(coords: Point, prev: &'a TrailStep) -> Self {
        TrailStep {
            coords,
            prev: Some(prev)
        }
    }
}

struct ScoreBoard {
    scores: HashMap<Point, usize>,
}

impl ScoreBoard {
    fn new() -> Self {
        ScoreBoard {
            scores: HashMap::new()
        }
    }

    fn get_score(&self, coords: &Point) -> Option<&usize> {
        self.scores.get(coords)
    }

    fn add(&mut self, coords: Point) {
        assert!(self.scores.insert(coords, 0).is_none())
    }

    fn update_scores(&mut self, increment: usize, trail_step: Option<&TrailStep>) {
        let mut current = trail_step;

        while let Some(step) = current {
            *self.scores.get_mut(&step.coords).unwrap() += increment;
            current = step.prev;
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

    fn is_valid_step(&self, from_coord: &Point, to_coord: &Point) -> bool {
        todo!()
    }

    fn find_trailheads(&self) {

        let mut scores = ScoreBoard::new();

        for start in self.bottoms.iter() {
            let mut trail: Vec<TrailStep> = vec![];
            let mut wavefront = vec![TrailStep::new(start.clone())];

            while !wavefront.is_empty() {
                let current = wavefront.pop().unwrap();

                if let Some(&score) = scores.get_score(&current.coords) {
                    if score > 0 {
                        scores.update_scores(score, current.prev)
                    }
                } else {
                    for c in current.coords.adjacent().into_iter()
                        .filter(|next_coords| self.is_valid_step(&current.coords, next_coords))
                    {
                    }
                }
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
