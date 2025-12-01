use std::collections::HashMap;

use shared::Problem;

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
        "240".to_string()
    }
    fn expected2(&self) -> String {
        "848076019766013".to_string()
    }
}
