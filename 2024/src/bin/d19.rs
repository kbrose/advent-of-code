use std::{collections::HashMap, fs};

fn parse_input(contents: &str) -> (Vec<String>, Vec<String>) {
    let (towels_str, designs_str) = contents.trim().split_once("\n\n").unwrap();
    let towels = towels_str.split(", ").map(|s| s.to_string()).collect();
    let designs = designs_str.split('\n').map(|s| s.to_string()).collect();
    (towels, designs)
}

fn satisfiable(cache: &mut HashMap<String, bool>, design: &str, towels: &Vec<String>) -> bool {
    if design.is_empty() {
        return true;
    }
    if cache.contains_key(design) {
        return cache[design];
    }
    let out = towels.iter().any(|towel| {
        if design.starts_with(towel) {
            satisfiable(cache, &design[towel.len()..], towels)
        } else {
            false
        }
    });
    cache.insert(design.to_string(), out);
    out
}

fn compute_1(contents: &str) -> usize {
    let (towels, designs) = parse_input(contents);
    let mut cache: HashMap<String, bool> = HashMap::new();
    designs
        .into_iter()
        .filter(|design| satisfiable(&mut cache, design, &towels))
        .count()
}

fn satisfiable_count(cache: &mut HashMap<String, u64>, design: &str, towels: &Vec<String>) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if cache.contains_key(design) {
        return cache[design];
    }
    let out = towels
        .iter()
        .map(|towel| {
            if design.starts_with(towel) {
                satisfiable_count(cache, &design[towel.len()..], towels)
            } else {
                0
            }
        })
        .sum();
    cache.insert(design.to_string(), out);
    out
}

fn compute_2(contents: &str) -> u64 {
    let (towels, designs) = parse_input(contents);
    let mut cache: HashMap<String, u64> = HashMap::new();
    designs
        .iter()
        .map(|design| satisfiable_count(&mut cache, design, &towels))
        .sum()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d19.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(240, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(848076019766013, result);
    println!("part 2: {result}");
}
