use std::io::{stdin, BufRead, BufReader, Read};

#[derive(Debug)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
}

impl Equation {
    fn is_valid_recursive(&self, so_far: i64, depth: usize, with_concat: bool) -> bool {
        if so_far > self.result {
            false
        }
        else if depth == self.operands.len() {
            so_far == self.result
        } else {
            let next_operand = self.operands[depth];

            (with_concat && self.is_valid_recursive((so_far.to_string() + &next_operand.to_string()).parse::<i64>().unwrap(), depth + 1, with_concat)) ||
            self.is_valid_recursive(so_far + next_operand, depth + 1, with_concat) ||
            self.is_valid_recursive(so_far * next_operand, depth + 1, with_concat)
        }
    }

    fn is_valid(&self, with_concat: bool) -> bool {
        self.is_valid_recursive(0, 0, with_concat)
    }
}

fn read_input<R>(stream: BufReader<R>) -> Vec<Equation>
    where R: Read,
{
    stream.lines()
        .map(|l| l.unwrap().split(':').map(|s| s.to_owned()).collect::<Vec<_>>())
        .map(|nums| {
            Equation {
                result: nums[0].parse::<i64>().unwrap(),
                operands: nums[1].trim().split_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect(),
            }
        })
        .collect()
}

fn main() {
    let eqs = read_input(BufReader::new(stdin()));

    let (simple_valid, simple_invalid): (Vec<&Equation>, Vec<&Equation>) =
        eqs.iter().partition(|&eq| eq.is_valid(false));

    let sum1 = simple_valid.iter()
        .map(|eq| eq.result)
        .sum::<i64>();
    eprintln!("Sum of valid equations: {sum1}");

    let sum2 = simple_invalid.iter()
        .filter(|&eq| eq.is_valid(true))
        .map(|eq| eq.result)
        .sum::<i64>();
    eprintln!("Sum of valid equations: {}", sum1 + sum2);
}
