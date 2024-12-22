use std::{
    collections::{HashMap, HashSet},
    fs,
};

const PRUNER: u64 = 2_u64.pow(24) - 1;

fn parse_input(contents: &str) -> Vec<u64> {
    contents
        .trim()
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect()
}

#[inline]
fn next(mut n: u64) -> u64 {
    // Step 1
    n = ((n << 6) ^ n) & PRUNER;
    // Step 2
    n = ((n >> 5) ^ n) & PRUNER;
    // Step 3
    ((n << 11) ^ n) & PRUNER
}

fn compute_1(contents: &str) -> u64 {
    let nums = parse_input(contents);
    let mut total = 0;
    for mut n in nums {
        for _ in 0..2000 {
            n = next(n);
        }
        total += n;
    }
    total
}

fn compute_2(contents: &str) -> u64 {
    let mut prices_after_sequence: HashMap<[i8; 4], Vec<u64>> = HashMap::new();
    let nums = parse_input(contents);
    let mut curr_sequence: [i8; 4] = [0; 4];
    for mut n in nums {
        let mut curr_price = n % 10;
        let mut observed_sequences_this_num: HashSet<[i8; 4]> = HashSet::new();
        for i in 0..2000 {
            n = next(n);
            let next_price = n % 10;
            for j in 0..3 {
                curr_sequence[j] = curr_sequence[j + 1];
            }
            curr_sequence[3] = (next_price as i8) - (curr_price as i8);
            if i >= 3 && !observed_sequences_this_num.contains(&curr_sequence) {
                prices_after_sequence
                    .entry(curr_sequence)
                    .or_default()
                    .push(next_price);
                observed_sequences_this_num.insert(curr_sequence);
            }
            curr_price = next_price;
        }
    }
    let mut best_observed_price_total = 0;
    for v in prices_after_sequence.values() {
        let sum = v.iter().sum();
        if sum > best_observed_price_total {
            best_observed_price_total = sum;
        }
    }
    best_observed_price_total
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d22.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(20506453102, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(2423, result);
    println!("part 2: {result}");
}
