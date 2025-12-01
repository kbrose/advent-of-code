use std::collections::HashMap;

use shared::Problem;

fn parse_input(contents: &str) -> Vec<u64> {
    contents
        .trim()
        .split(' ')
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn num_digits(n: u64) -> u32 {
    // I'm pretty sure there's a bitwise solution that could be done as well,
    // same as can be done for log2, but this is a lot quicker to write.
    let mut counter = 1;
    let mut i = 10;
    while i <= n {
        i *= 10;
        counter += 1;
    }
    counter
}

fn split_stone(mut stone: u64) -> Option<(u64, u64)> {
    let digit_count = num_digits(stone);
    if digit_count % 2 == 0 {
        let mut n1 = 0;
        for i in 0..(digit_count / 2) {
            n1 += (stone % 10) * (10_u64.pow(i));
            stone /= 10;
        }
        let n2 = stone;
        Some((n1, n2))
    } else {
        None
    }
}

enum NextStep {
    Singular(u64),
    Split(u64, u64),
}

fn next_stone(stone: u64) -> NextStep {
    if stone == 0 {
        NextStep::Singular(1)
    } else if let Some((n1, n2)) = split_stone(stone) {
        NextStep::Split(n1, n2)
    } else {
        NextStep::Singular(stone * 2024)
    }
}

fn stone_counts_after_blinks(
    stone: u64,
    blinks_left: u8,
    cache: &mut HashMap<(u64, u8), u64>,
) -> u64 {
    if cache.contains_key(&(stone, blinks_left)) {
        return cache[&(stone, blinks_left)];
    }
    let out = {
        if blinks_left == 0 {
            1
        } else {
            match next_stone(stone) {
                NextStep::Singular(s) => stone_counts_after_blinks(s, blinks_left - 1, cache),
                NextStep::Split(s1, s2) => {
                    stone_counts_after_blinks(s1, blinks_left - 1, cache)
                        + stone_counts_after_blinks(s2, blinks_left - 1, cache)
                }
            }
        }
    };
    cache.entry((stone, blinks_left)).or_insert(out);
    out
}

fn solve(stones: Vec<u64>, steps: u8) -> u64 {
    let mut cache = HashMap::new();
    stones
        .into_iter()
        .map(|stone| stone_counts_after_blinks(stone, steps, &mut cache))
        .sum()
}

fn compute_1(contents: &str) -> u64 {
    solve(parse_input(contents), 25)
}

fn compute_2(contents: &str) -> u64 {
    solve(parse_input(contents), 75)
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
        "217443".to_string()
    }
    fn expected2(&self) -> String {
        "257246536026785".to_string()
    }
}
