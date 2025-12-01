use shared::Problem;
use std::collections::HashMap;

fn parse_input(contents: &str) -> (Vec<u64>, Vec<u64>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line in contents.trim().split('\n') {
        let (a, b) = line.split_once("   ").unwrap();
        v1.push(a.parse().unwrap());
        v2.push(b.parse().unwrap());
    }
    (v1, v2)
}

fn compute_1(contents: &str) -> u64 {
    let (mut v1, mut v2) = parse_input(contents);
    v1.sort();
    v2.sort();
    (0..v1.len())
        .map(|i| std::cmp::max(v1[i], v2[i]) - std::cmp::min(v1[i], v2[i]))
        .sum()
}

fn compute_2(contents: &str) -> u64 {
    let (v1, v2) = parse_input(contents);
    let mut counter: HashMap<u64, u64> = HashMap::new();
    for value in v2.iter() {
        *counter.entry(*value).or_insert(0) += 1;
    }
    v1.iter().map(|i| i * counter.get(i).unwrap_or(&0)).sum()
}

pub(crate) struct Day {}

impl Problem for Day {
    fn source_code_file(&self) -> String {
        file!().to_string()
    }
    fn solve1(&self, contents: &str) -> String {
        format!("{}", compute_1(contents))
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected1(&self) -> String {
        "1320851".to_string()
    }
    fn expected2(&self) -> String {
        "26859182".to_string()
    }
}
