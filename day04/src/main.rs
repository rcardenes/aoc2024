use std::io::{stdin, BufRead, BufReader, Read};
use itertools::izip;

#[derive(Debug)]
enum Diagonal {
    TopDownFwd,
    TopDownBkd,
    BottomUpFwd,
    BottomUpBkd,
}

struct Board {
    lines: Vec<Vec<char>>,
    x_map: Vec<(i32, i32)>,
    a_map: Vec<(i32, i32)>,
    cols: usize,
    rows: usize,
}

impl Board {
    fn new(lines: Vec<String>, x_map: Vec<(i32, i32)>, a_map: Vec<(i32, i32)>) -> Board {
        let lines = lines.into_iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        Board {
            rows: lines.len(),
            cols: lines[0].len(),
            lines,
            x_map,
            a_map,
        }
    }

    fn count_xmas(&self) -> usize {
        let mut count = 0;

        for &(x_row, x_col) in self.x_map.iter() {
            if self.in_horiz(x_row as usize, x_col, x_col + 3, false) { count += 1; };
            if self.in_horiz(x_row as usize, x_col - 3, x_col, true) { count += 1; };
            if self.in_vert(x_col as usize, x_row, x_row + 3, false) { count += 1; };
            if self.in_vert(x_col as usize, x_row - 3, x_row, true) { count += 1; };
            if self.in_diag(x_row, x_col, Diagonal::TopDownFwd) { count += 1; };
            if self.in_diag(x_row, x_col, Diagonal::BottomUpFwd) { count += 1; };
            if self.in_diag(x_row, x_col, Diagonal::TopDownBkd) { count += 1; };
            if self.in_diag(x_row, x_col, Diagonal::BottomUpBkd) { count += 1; };
        }

        count
    }

    fn count_x_mas(&self) -> usize {
        let mut count = 0;

        for &(x_row, x_col) in self.a_map.iter() {
            let (r1, c1, r2, c2) = (x_row - 1, x_col - 1, x_row as usize + 1, x_col as usize + 1);
            if r1 < 0 || c1 < 0 || r2 >= self.rows || c2 >= self.cols {
                continue
            }

            let (r1, c1) = (r1 as usize, c1 as usize);
            //  M
            //   A
            //    S
            let case_1 = self.lines[r1][c1] == 'M' && self.lines[r2][c2] == 'S';
            //  S
            //   A
            //    M
            let case_2 = self.lines[r1][c1] == 'S' && self.lines[r2][c2] == 'M';

            //    M
            //   A
            //  S  
            let case_3 = self.lines[r1][c2] == 'M' && self.lines[r2][c1] == 'S';
            //    S
            //   A
            //  M  
            let case_4 = self.lines[r1][c2] == 'S' && self.lines[r2][c1] == 'M';

            if (case_1 || case_2) && (case_3 || case_4){
                count += 1;
            }
        }

        count
    }

    fn in_horiz(&self, row: usize, a: i32, b: i32, rev: bool) -> bool {
        if a < 0 || (b as usize) >= self.cols {
            false
        } else {
            let comp = if rev {"SAMX"} else {"XMAS"};
            self.lines[row][a as usize..=b as usize].iter().zip(comp.chars())
                .all(|(&c1, c2)| c1 == c2)
        }
    }
    fn in_vert(&self, col: usize, a: i32, b: i32, rev: bool) -> bool {
        if a < 0 || (b as usize) >= self.rows {
            false
        } else {
            let comp = if rev {"SAMX"} else {"XMAS"};
            (a as usize..=b as usize).zip(comp.chars())
                .all(|(row, c)| {
                    let k = col;
                    self.lines[row][k] == c
                })
        }
    }

    fn in_diag(&self, row: i32, col: i32, kind: Diagonal) -> bool {
        let (r1, c1, r2, c2) = match kind {
            Diagonal::TopDownFwd => (row, col, row+3, col+3),
            Diagonal::TopDownBkd => (row-3, col-3, row, col),
            Diagonal::BottomUpFwd => (row, col, row-3, col+3),
            Diagonal::BottomUpBkd => (row+3, col-3, row, col),
        };
        if r1 < 0 || r2 < 0 || r1 as usize >= self.rows || r2 as usize >= self.rows {
            false
        } else if c1 < 0 || c2 < 0 || c1 as usize >= self.cols || c2 as usize >= self.cols {
            false
        } else {
            let (r_range, c_range): (Vec<usize>, Vec<usize>) = match kind {
                Diagonal::TopDownFwd => ((r1 as usize..=r2 as usize).collect(), (c1 as usize..=c2 as usize).collect()),
                Diagonal::TopDownBkd => ((r1 as usize..=r2 as usize).rev().collect(), (c1 as usize..=c2 as usize).rev().collect()),
                Diagonal::BottomUpFwd => ((r2 as usize..=r1 as usize).rev().collect(), (c1 as usize..=c2 as usize).collect()),
                Diagonal::BottomUpBkd => ((r2 as usize..=r1 as usize).collect(), (c1 as usize..=c2 as usize).rev().collect()),
            };

            izip!(r_range, c_range, "XMAS".chars()) 
                .all(|(row, col, c)| self.lines[row][col] == c)
        }
    }
}

fn read_input<R>(stream: BufReader<R>) -> Board
    where R: Read,
{
    let mut x_map = vec![];
    let mut a_map = vec![];
    let mut lines = vec![];
    for (row, line) in stream.lines().enumerate() {
        let line = line.unwrap();
        line.chars().enumerate()
            .for_each(|(col, c)| {
                if c == 'X' {
                    x_map.push((row as i32, col as i32));
                } else if c == 'A' {
                    a_map.push((row as i32, col as i32));
                }
            });
        lines.push(line.trim_end().to_string());
    }

    Board::new(lines, x_map, a_map)
}

fn main() {
    let board = read_input(BufReader::new(stdin()));
    let n_xmas = board.count_xmas();
    let n_x_mas = board.count_x_mas();

    println!("Number of XMAS: {n_xmas}");
    println!("Number of X-MAS: {n_x_mas}");
}
