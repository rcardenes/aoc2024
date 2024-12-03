use std::io::{stdin, BufRead, BufReader, Read};

#[derive(Debug)]
struct Mul {
    a: usize,
    b: usize,
}

impl Mul {
    fn result(&self) -> usize {
        self.a * self.b
    }
}

#[derive(Debug)]
enum LexerState {
    Searching,
    M,
    U,
    L,
    LParen,
    Comma,
    Num1,
    Num2,
}

fn extract_mults(string: &str) -> Vec<Mul> {
    let mut state = LexerState::Searching;
    let mut mults = vec![];
    let mut num1 = String::new();
    let mut num2 = String::new();

    for c in string.chars() {
        match state {
            LexerState::Searching => { if c == 'm' { num1.clear(); num2.clear(); state = LexerState::M } },
            LexerState::M => state = if c == 'u' { LexerState::U } else { LexerState::Searching },
            LexerState::U => state = if c == 'l' { LexerState::L } else { LexerState::Searching },
            LexerState::L => state = if c == '(' { LexerState::LParen } else { LexerState::Searching },
            LexerState::LParen => state = if c.is_ascii_digit() { num1.push(c); LexerState::Num1 } else { LexerState::Searching },
            LexerState::Comma => state = if c.is_ascii_digit() { num2.push(c); LexerState::Num2 } else { LexerState::Searching },
            LexerState::Num1 => state = match c {
                                    ',' => LexerState::Comma,
                                    '0'..='9' if num1.len() < 3 => { num1.push(c); LexerState::Num1 },
                                    _ => LexerState::Searching,
                                },
            LexerState::Num2 => state = match c {
                                    ')' => {
                                        mults.push(Mul {
                                            a: num1.parse::<usize>().unwrap(),
                                            b: num2.parse::<usize>().unwrap(),
                                        });
                                        LexerState::Searching
                                    },
                                    '0'..='9' if num2.len() < 3 => { num2.push(c); LexerState::Num2 },
                                    _ => LexerState::Searching,
                                },
        }
    }

    mults
}

fn read_samples<R>(stream: BufReader<R>) -> Vec<String>
    where R: Read,
{
    stream.lines().map(|l| l.unwrap().trim().to_string()).collect()
}

fn main() {
    let input = read_samples(BufReader::new(stdin()));
    let mul_groups = input.iter()
                        .map(|l| extract_mults(&l))
                        .collect::<Vec<_>>();
    let mul_sums = mul_groups.iter()
                             .map(|muls| muls.iter().map(|m| m.result()).sum::<usize>())
                             .sum::<usize>();
    println!("Multiplications added: {mul_sums:?}");
}
