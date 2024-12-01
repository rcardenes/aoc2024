use std::io::{self, BufRead, BufReader};

fn first_half(l1: &mut Vec<i64>, l2: &mut Vec<i64>) {
    let sum_diffs: i64 = l1.iter().zip(l2.iter())
                        .map(|(&a, &b)| { (a - b).abs() })
                        .sum();
    println!("Sum of distances: {sum_diffs}");
}

fn read_lists<R>(stream: R) -> (Vec<i64>, Vec<i64>)
    where R: BufRead
{
    let mut l1 = vec![];
    let mut l2 = vec![];

    for line in stream.lines() {
        let k = line.unwrap().trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        l1.push(k[0]);
        l2.push(k[1]);
    }

    (l1, l2)
}

fn main() {
    let stream = BufReader::new(std::io::stdin());
    let (mut l1, mut l2) = read_lists(stream);

    l1.sort();
    l2.sort();

    first_half(&mut l1, &mut l2);
}
