use std::{collections::{HashMap, HashSet}, io::{stdin, BufRead, BufReader, Read}};

fn is_right(list: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut seen = HashSet::new();

    for v in list {
        if let Some(ruleset) = rules.get(v) {
            if seen.iter().any(|k| ruleset.contains(k)) {
                return false
            }
        }
        seen.insert(*v);
    }

    true
}

fn fix_list(list: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    let mut result =  vec![];

    result.push(list[0]);

    for &k in &list[1..] {
        let k_rules = rules.get(&k);
        let mut insert_at: Option<usize> = None;

        for (i, v) in result.iter().enumerate() {
            if k_rules.is_some_and(|order| order.contains(v)) {
                insert_at = Some(i);
                break;
            }
        }

        if let Some(index) = insert_at {
            result.insert(index, k);
        } else {
            result.push(k);
        }
    }

    result
}

fn read_rules<R>(stream: BufReader<R>) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>)
    where R: Read,
{
    let lines = stream.lines().into_iter();
    let mut rules = HashMap::new();
    let mut lists = vec![];
    let mut collecting_lists = false;

    for line in lines {
        let line = line.unwrap();
        if line.trim().is_empty() {
            collecting_lists = true;
        } else if collecting_lists {
            lists.push(line.trim().split(',').map(|v| v.parse::<usize>().unwrap()).collect());
        } else {
            let bits = line.trim().split('|').collect::<Vec<_>>();
            let (a, b) = (bits[0].parse::<usize>().unwrap(), bits[1].parse::<usize>().unwrap());

            rules.entry(a).or_insert(HashSet::new()).insert(b);
        }
    }

    (rules, lists)
}

fn main() {
    let (rules, printing_lists) = read_rules(BufReader::new(stdin()));

    let (well_ordered, badly_ordered): (Vec<_>, Vec<_>) = printing_lists.into_iter()
        .partition(|list| is_right(list, &rules));

    let middle_page_sum = well_ordered.iter()
        .map(|list| list[list.len() / 2])
        .sum::<usize>();

    println!("Sum of ordered lists: {middle_page_sum}");

    let middle_page_sum = badly_ordered.iter()
        .map(|list| fix_list(list, &rules))
        .map(|list| list[list.len() / 2])
        .sum::<usize>();

    println!("Sum of reordered lists: {middle_page_sum}");
}
