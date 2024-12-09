use std::{collections::{HashMap, HashSet}, io::{stdin, BufRead, BufReader, Read}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coords {
    x: i64,
    y: i64,
}

impl Coords {
    fn from_input(row: usize, col: usize) -> Self {
        Coords { x: col as i64, y: row as i64 }
    }

    fn distance(&self, other: &Coords) -> (i64, i64) {
        (other.y - self.y, other.x - self.x)
    }

    fn add(&self, dy: i64, dx: i64) -> Coords {
        Coords {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn antinodes(&self, other: &Coords) -> Vec<Coords> {
        let (dy, dx) = self.distance(other);

        vec![
            Coords { x: self.x - dx, y: self.y - dy },
            Coords { x: other.x + dx, y: other.y + dy },
        ]
    }
}

#[derive(Debug)]
struct Map {
    width: i64,
    height: i64,
//    antennas: HashSet<Coords>,
    antennas_by_frequency: HashMap<char, Vec<Coords>>
}

//fn generate(w: usize) -> String {
//    (0..w).map(|n| (n % 10).to_string()).collect::<Vec<_>>().join("")
//}
//
impl Map {
    fn contains(&self, coords: &Coords) -> bool {
        coords.x >= 0 && coords.x < self.width && coords.y >= 0 && coords.y < self.height
    }

    //fn print_antinodes(&self, antinodes: &HashSet<Coords>) {
    //    let mut lines = vec![];
    //    let width = self.width as usize;
    //    let row = format!("{:.<width$}", "");
    //
    //    for _ in 0..self.height {
    //        lines.push(row.clone());
    //    }
    //
    //    println!("   {}", generate(self.width as usize));
    //
    //    for (&c, coords) in self.antennas_by_frequency.iter() {
    //        for coord in coords {
    //            unsafe {
    //                let line = lines[coord.y as usize].as_bytes_mut(); // [an.x as usize] = '#';
    //                line[coord.x as usize] = c as u8;
    //            }
    //        }
    //    }
    //
    //    for an in antinodes {
    //        unsafe {
    //            let line = lines[an.y as usize].as_bytes_mut(); // [an.x as usize] = '#';
    //            line[an.x as usize] = b'#';
    //       }
    //    }
    //
    //    for (row, l) in lines.into_iter().enumerate() {
    //        println!("{row:2} {l}")
    //    }
    //}
}

fn find_antinodes_with_harmonics(map: &Map) -> usize {
    let mut antinodes = HashSet::new();

    for (_, nodes) in map.antennas_by_frequency.iter() {
        for (index, a) in nodes.split_last().unwrap().1.iter().enumerate() {
            for b in &nodes[index+1..] {
                let (dy, dx) = a.distance(b);
                antinodes.insert(a.clone());
                antinodes.insert(b.clone());

                let mut antinode = a.add(-dy, -dx);
                while map.contains(&antinode) {
                    antinodes.insert(antinode.clone());
                    antinode = antinode.add(-dy, -dx);
                }

                let mut antinode = b.add(dy, dx);
                while map.contains(&antinode) {
                    antinodes.insert(antinode.clone());
                    antinode = antinode.add(dy, dx);
                }
            }
        }
    }

    antinodes.len()
}

fn find_antinodes(map: &Map) -> usize {
    let mut antinodes = HashSet::new();

    for (_, nodes) in map.antennas_by_frequency.iter() {
        for (index, a) in nodes.split_last().unwrap().1.iter().enumerate() {
            for b in &nodes[index+1..] {
                for an in a.antinodes(b).into_iter().filter(|c| map.contains(c)) {
                    antinodes.insert(an);
                }
            }
        }
    }

    antinodes.len()
}


fn read_map<R>(stream: BufReader<R>) -> Map
    where R: Read,
{
    let mut _antennas: HashSet<Coords> = HashSet::new();
    let mut antennas_by_frequency: HashMap<char, Vec<Coords>> = HashMap::new();

    let lines = stream.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let width = lines.len() as i64;
    let height = lines[0].len() as i64;

    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
            antennas_by_frequency.entry(c)
                .and_modify(|v| v.push(Coords::from_input(row, col)))
                .or_insert(vec![Coords::from_input(row, col)]);
            _antennas.insert(Coords::from_input(row, col));
        }
    }

    Map {
        width,
        height,
//        antennas,
        antennas_by_frequency,
    }
}

fn main() {
    let map = read_map(BufReader::new(stdin()));

    let n_antinodes = find_antinodes(&map);
    println!("Total antinodes: {n_antinodes}");

    let n_antinodes = find_antinodes_with_harmonics(&map);
    println!("Total antinodes: {n_antinodes}");
}
