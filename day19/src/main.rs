use std::{
    io::{stdin, BufRead, BufReader, Read},
    slice::Iter,
};

#[derive(Clone)]
struct Subproblem<'a> {
    prefix_length: usize,
    target: &'a str,
    patterns: Iter<'a, String>,
}

impl<'a> Subproblem<'a> {
    fn new(prefix_length: usize, target: &'a str, patterns: Iter<'a, String>) -> Self {
        Subproblem {
            prefix_length,
            target,
            patterns,
        }
    }
}

fn is_design_possible(design: &str, patterns: &Vec<String>) -> bool {
    let mut subproblems = vec![Subproblem::new(0, design, patterns.iter())];

    while !subproblems.is_empty() {
        let mut current = subproblems.pop().unwrap();

        while let Some(pattern) = current.patterns.next() {
            if current.target == pattern {
                return true;
            } else if current.target.starts_with(pattern) {
                let sub_prefix = current.prefix_length + pattern.len();
                subproblems.push(current.clone());
                subproblems.push(Subproblem {
                    prefix_length: sub_prefix,
                    target: &design[sub_prefix..],
                    patterns: patterns.iter(),
                });
            }
        }
    }

    false
}

fn read_input<R>(stream: BufReader<R>) -> (Vec<String>, Vec<String>)
where 
    R: Read
{
    let mut lines = stream.lines()
        .map(|l| l.unwrap());

    let patterns = lines.next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let _ = lines.next(); // Ignore the blank
    let designs = lines.collect();

    (patterns, designs)
}

fn main() {
    let (patterns, designs) = read_input(BufReader::new(stdin()));

    let possible_designs = designs.iter().filter(|d| is_design_possible(d, &patterns)).count();

    println!("Number of possible designs: {possible_designs}");
}
