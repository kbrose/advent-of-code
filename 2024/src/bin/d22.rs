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
    // Step 2 - note, no actual need to do (& PRUNER) here.
    n = (n >> 5) ^ n;
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
    let mut prices_after_sequence: HashMap<u32, u64> = HashMap::new();
    let nums = parse_input(contents);
    let mut best_observed_price_total = 0;
    // We're going to bit pack the sequence into the "curr_sequence" variable.
    // Consider the binary layout 0b_aaaaa_bbbbb_ccccc_ddddd.
    // The aaaaa bits will correspond to the first delta in the sequence,
    // bbbbb the second, etc.
    let mut curr_sequence = 0;
    for mut n in nums {
        let mut curr_price = n % 10;
        let mut observed_sequences_this_num: HashSet<u32> = HashSet::new();
        for i in 0..2000 {
            n = next(n);
            let next_price = n % 10;
            curr_sequence <<= 5;
            curr_sequence &= 0b_11111_11111_11111_11111;
            // the (absolute) difference is between 0 and 9 inclusive, so
            // it will fit in the first four bits of curr_sequence
            curr_sequence += next_price.abs_diff(curr_price) as u32;
            if next_price > curr_price {
                // If necessary, a sign bit is added at the fifth bit.
                curr_sequence += 0b_10000;
            }
            if i >= 3 && !observed_sequences_this_num.contains(&curr_sequence) {
                let price = prices_after_sequence.entry(curr_sequence).or_default();
                *price += next_price;
                best_observed_price_total = std::cmp::max(*price, best_observed_price_total);
                observed_sequences_this_num.insert(curr_sequence);
            }
            curr_price = next_price;
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
