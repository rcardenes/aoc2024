use std::{collections::{HashMap, HashSet}, io::{stdin, BufRead, BufReader, Read}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_col_row(col: usize, row: usize) -> Point {
        Point { x: col as i32, y: row as i32 }
    }

    fn neighbors(&self) -> Vec<Point> {
        vec![
            Point { x: self.x + 1, y: self.y },
            Point { x: self.x - 1, y: self.y },
            Point { x: self.x, y: self.y + 1 },
            Point { x: self.x, y: self.y - 1 },
        ]
    }

    fn neighbor_set(&self) -> HashSet<Point> {
        HashSet::from_iter(self.neighbors().into_iter())
    }
}

#[derive(Debug)]
struct Region {
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

    fn fencing_cost(&self) -> usize {
        self.area() * self.perimeter()
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
    
    println!("Fencing costs: {}", regions.iter().map(Region::fencing_cost).sum::<usize>());
}
