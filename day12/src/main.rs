use std::{collections::{HashMap, HashSet}, io::{stdin, BufRead, BufReader, Read}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    const VALUES: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.y.cmp(&self.y) {
            std::cmp::Ordering::Equal => other.x.cmp(&self.x),
            cmp_res => cmp_res,
        }
    }
}

impl Point {
    fn from_col_row(col: usize, row: usize) -> Point {
        Point { x: col as i32, y: row as i32 }
    }

    fn neighbors(&self) -> Vec<Point> {
        Vec::from(Direction::VALUES.map(|facing| self.neighbor(facing)))
    }

    fn neighbor(&self, facing: Direction) -> Point {
        match facing {
            Direction::Up => Point { x: self.x, y: self.y - 1 },
            Direction::Down => Point { x: self.x, y: self.y + 1 },
            Direction::Left => Point { x: self.x - 1, y: self.y },
            Direction::Right => Point { x: self.x + 1, y: self.y },
        }
    }

    fn neighbor_set(&self) -> HashSet<Point> {
        HashSet::from_iter(self.neighbors().into_iter())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Side {
    start: Point,
    end: Point,
}

impl Side {
    fn adjacent(&self, side: &Direction, other: &Point) -> bool {
        match side {
            Direction::Up | Direction::Down => { self.end.y == other.y && (self.end.x - other.x).abs() == 1 },
            Direction::Left | Direction::Right => { self.end.x == other.x && (self.end.y - other.y).abs() == 1 },
        }
    }

    fn absorb(&mut self, other: Point) {
        if self.end.x < other.x || self.end.y < other.y {
            self.end = other;
        } else {
            self.start = other;
        }
    }
}

impl From<Point> for Side {
    fn from(value: Point) -> Self {
        Side {
            start: value.clone(),
            end: value,
        }
    }
}

impl PartialOrd for Side {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Side {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

#[derive(Debug)]
struct Region {
    #[allow(dead_code)]
    plant: char,
    plots: Vec<Point>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        let plot_set = HashSet::from_iter(self.plots.iter().cloned());

        self.plots
            .iter()
            .map(|p| p.neighbor_set().difference(&plot_set).count())
            .sum::<usize>()
    }

    fn sides(&self) -> usize {
        let plot_set: HashSet<Point> = HashSet::from_iter(self.plots.iter().cloned());

        let mut plots_by_facing: HashMap<Direction, Vec<Point>> = HashMap::new();

        for plot in plot_set.iter() {
            Direction::VALUES.iter()
                .filter(|&facing| !plot_set.contains(&plot.neighbor(facing.clone())))
                .for_each(|facing| {
                    plots_by_facing
                        .entry(facing.clone())
                        .and_modify(|v| v.push(plot.clone()))
                        .or_insert(vec![plot.clone()]);
                });
        }

        let mut num_sides = 0usize;

        for (dir, mut plots) in plots_by_facing.into_iter() {
            plots.sort();
            let mut fused = vec![Side::from(plots.pop().unwrap())];

            while let Some(plot) = plots.pop() {
                if let Some(matching) = fused.iter_mut().skip_while(|s| !s.adjacent(&dir, &plot)).next() {
                    matching.absorb(plot);
                } else {
                    fused.push(Side::from(plot))
                }
            }

            num_sides += fused.len();
        }


        num_sides
    }

    fn fencing_cost(&self, discounted: bool) -> usize {
        self.area() * if discounted { self.sides() } else { self.perimeter() }
    }
}

struct Map {
    n_cols: usize,
    n_rows: usize,
    rows: Vec<Vec<char>>,
}

impl Map {
    fn matches_plant(&self, plant: char, plot: &Point) -> bool {
        plot.x >= 0 && plot.y >= 0
        && plot.x < self.n_cols as i32 && plot.y < self.n_rows as i32
        && self.rows[plot.y as usize][plot.x as usize] == plant
    }

    fn flood_find_region(&self, seed: &Point) -> (Region, HashSet<Point>) {
        let plant = self.rows[seed.y as usize][seed.x as usize];
        let mut plots = vec![];
        let mut visited: HashSet<Point> = HashSet::new();
        let mut candidates = vec![seed.clone()];

        while let Some(plot) = candidates.pop() {
            if !visited.contains(&plot) {
                visited.insert(plot.clone());

                if self.matches_plant(plant, &plot) {
                    candidates.extend(plot.neighbors());
                    plots.push(plot);
                }
            }
        }

        let plot_set = HashSet::from_iter(plots.iter().cloned());
        (Region { plant, plots }, plot_set)
    }

    fn generate_regions(&self) -> Vec<Region> {
        let mut found: HashSet<Point> = HashSet::new();
        let mut regions = vec![];

        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
                let plot = Point::from_col_row(col, row);
                if !found.contains(&plot) {
                    let (region, visited) = self.flood_find_region(&plot);
                    regions.push(region);
                    found.extend(visited);
                }
            }
        }

        regions
    }
}

fn read_input<R>(stream: BufReader<R>) -> Map
where 
    R: Read
{

    let rows = stream.lines()
        .into_iter()
        .map(|l| l.expect("I expected an input without errors!")
            .trim_end()
            .chars()
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Map {
        n_rows: rows.len(),
        n_cols: rows[0].len(),
        rows
    }
}

fn main() {
    let map = read_input(BufReader::new(stdin()));
    let regions = map.generate_regions();
    
    println!("Fencing costs:            {}", regions.iter().map(|r| r.fencing_cost(false)).sum::<usize>());
    println!("Discounted fencing costs: {}", regions.iter().map(|r| r.fencing_cost(true)).sum::<usize>());
}
