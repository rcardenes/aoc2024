use std::{collections::HashMap, io::{stdin, BufRead, BufReader, Read}, ops::Deref};

#[derive(Clone, Debug)]
struct Stone(usize);

impl ToString for Stone {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<&str> for Stone {
    fn from(value: &str) -> Self {
        Self(value.parse::<usize>().unwrap())
    }
}

impl Stone {
    fn is_even_digits(&self) -> bool {
        (self.to_string().len() % 2) == 0
    }

    fn blink(&self) -> Vec<Stone> {
        match self.0 {
            0 => vec![Stone(1)],
            _ if self.is_even_digits() => {
                let s = self.to_string();
                let (l, r) = s.split_at(s.len() / 2);
                vec![l.into(), r.into()]
            }
            _ => vec![Stone(self.0 * 2024)],
        }
    }
}

impl Deref for Stone {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Blinker {
    memo: HashMap<(usize, usize), usize>
}

impl Blinker {
    fn new() -> Self {
        Blinker { memo: HashMap::new() }
    }

    fn blink_single(&mut self, stone: &Stone, times: usize) -> usize {
        match times {
            0 => 1,
            1 => stone.blink().len(),
            _ => {
                let stone_val = *stone.deref();
                if let Some(value) = self.memo.get(&(stone_val, times)) {
                    *value
                } else {
                    let value = stone.blink().iter()
                        .map(|s| self.blink_single(s, times - 1))
                        .sum::<usize>();
                    self.memo.insert((stone_val, times), value);

                    value
                }
            }
        }
    }

    fn after_blinking_times(&mut self, stones: &Vec<Stone>, times: usize) -> usize {
        stones.iter().map(|s| self.blink_single(s, times)).sum::<usize>()
    }
}

fn read_input<R>(mut stream: BufReader<R>) -> Vec<Stone>
    where R: Read
{
    let mut line = String::new();
    stream.read_line(&mut line).expect("I expected a properly working stream!");

    line.split_whitespace().map(Stone::from).collect()
}

fn main() {
    let initial_stones = read_input(BufReader::new(stdin()));
    let mut blinker = Blinker::new();

    eprintln!("After 25: {}", blinker.after_blinking_times(&initial_stones, 25));
    eprintln!("After 75: {}", blinker.after_blinking_times(&initial_stones, 75));
    // let after_25 = initial_stones.iter().
    // eprintln!("After 25: {}", after_25.len());
    // let after_75 = (0..50).fold(initial_stones.clone(), |acc, _| acc.iter().flat_map(|s| s.blink()).collect::<Vec<_>>());
    // eprintln!("After 75: {}", after_75.len());
}
