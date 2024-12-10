use std::fs;

fn parse_input(contents: &str) -> Vec<Vec<i64>> {
    contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|entry| entry.parse().unwrap())
                .collect()
        })
        .collect()
}

#[allow(clippy::ptr_arg)]
fn find_next(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|n| n == &seq[0]) {
        seq[0]
    } else {
        seq.last().unwrap()
            + find_next(
                &seq.iter()
                    .zip(seq.iter().skip(1))
                    .map(|(a, b)| b - a)
                    .collect(),
            )
    }
}

#[allow(clippy::ptr_arg)]
fn find_prev(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|n| n == &seq[0]) {
        seq[0]
    } else {
        seq.first().unwrap()
            - find_prev(
                &seq.iter()
                    .zip(seq.iter().skip(1))
                    .map(|(a, b)| b - a)
                    .collect(),
            )
    }
}

fn compute_1(contents: &str) -> i64 {
    let sequences = parse_input(contents);

    sequences.iter().map(find_next).sum()
}

fn compute_2(contents: String) -> i64 {
    let sequences = parse_input(&contents);

    sequences.iter().map(find_prev).sum()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d09.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(1479011877, result);
    println!("part 1: {result}");

    let result = compute_2(contents);
    assert_eq!(973, result);
    println!("part 2: {result}");
}
