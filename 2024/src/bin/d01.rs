use std::{collections::HashMap, fs};

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

fn main() {
    let contents =
        fs::read_to_string("inputs/d01.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(1320851, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(26859182, result);
    println!("part 2: {result}");
}
