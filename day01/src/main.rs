use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader}
};

fn first_half(l1: &Vec<i64>, l2: &Vec<i64>) {
    let sum_diffs= l1.iter().zip(l2.iter())
                        .map(|(&a, &b)| { (a - b).abs() })
                        .sum::<i64>();
    println!("Sum of distances: {sum_diffs}");
}

fn second_half(l1: &Vec<i64>, l2: &Vec<i64>) {
    let mapping = l2.into_iter().dedup_with_count().map(|(a, &b)| (b, a)).collect::<HashMap<_, _>>();
    let score = l1.into_iter()
                        .map(|val| mapping.get(val).map_or(0, |&times| (*val as usize) * times))
                        .sum::<usize>();


    println!("Similarity score: {score}");
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

    first_half(&l1, &l2);
    second_half(&l1, &l2);
}
