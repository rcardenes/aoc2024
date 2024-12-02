use std::io::{stdin, BufRead, BufReader, Read};

#[derive(Debug)]
struct Report {
    levels: Vec<i64>,
}

impl Report {
    fn from_str(string: &str) -> Report {
       Report {
            levels: string.trim().split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect()
        }
    }

    fn all_increasing(&self) -> bool {
        let n = self.levels.len();

        for (&a, &b) in self.levels[0..(n-1)].iter().zip(self.levels[1..].iter()) {
            let diff = b - a;
            if diff < 1 || diff > 3 {
                return false;
            }
        }

        true
    }

    fn all_decreasing(&self) -> bool {
        let n = self.levels.len();

        for (&a, &b) in self.levels[0..(n-1)].iter().zip(self.levels[1..].iter()) {
            let diff = a - b;
            if diff < 1 || diff > 3 {
                return false;
            }
        }

        true
    }

    fn is_safe(&self) -> bool {
        self.all_increasing() || self.all_decreasing()
    }
}

fn read_reports<R>(stream: BufReader<R>) -> Vec<Report>
    where R: Read
{
    stream.lines()
        .map(|l| Report::from_str(&l.unwrap()))
        .collect()
}

fn main() {
    let reports = read_reports(BufReader::new(stdin()));

    let number_safe = reports.iter().filter(|&r| r.is_safe()).count();

    println!("# safe reports: {number_safe}");
}
