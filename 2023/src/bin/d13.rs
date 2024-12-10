use std::fs;

type Pattern = Vec<Vec<bool>>;

fn transpose(pattern: &Pattern) -> Pattern {
    let mut new = vec![Vec::with_capacity(pattern.len()); pattern[0].len()];
    for pattern_i in pattern.iter() {
        for (j, pattern_ij) in pattern_i.iter().enumerate() {
            new[j].push(*pattern_ij);
        }
    }
    new
}

fn parse_input(contents: &str) -> Vec<Pattern> {
    contents
        .trim_end()
        .split("\n\n")
        .map(|lines| {
            lines
                .trim()
                .split('\n')
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect()
}

fn horizontal_symmetry(pattern: &Pattern) -> Option<usize> {
    for closest_row_above in 0..(pattern.len() - 1) {
        if (0..closest_row_above + 1)
            .rev()
            .zip(closest_row_above + 1..pattern.len())
            .all(|(row_above, row_below)| pattern[row_above] == pattern[row_below])
        {
            return Some(closest_row_above + 1);
        }
    }
    None
}

fn compute_1(contents: &str) -> usize {
    let mut summand = 0;
    for pattern in parse_input(contents) {
        match horizontal_symmetry(&pattern) {
            Some(val) => {
                summand += val * 100;
            }
            None => {
                summand += horizontal_symmetry(&transpose(&pattern)).unwrap();
            }
        }
    }
    summand
}

fn diff_count(seq1: &[bool], seq2: &[bool]) -> u32 {
    seq1.iter()
        .zip(seq2.iter())
        .map(|(a, b)| (a != b) as u32)
        .sum()
}

fn horizontal_almost_symmetry(pattern: &Pattern) -> Option<usize> {
    for closest_row_above in 0..(pattern.len() - 1) {
        if (0..closest_row_above + 1)
            .rev()
            .zip(closest_row_above + 1..pattern.len())
            .map(|(row_above, row_below)| diff_count(&pattern[row_above], &pattern[row_below]))
            .sum::<u32>()
            == 1
        {
            return Some(closest_row_above + 1);
        }
    }
    None
}

fn compute_2(contents: &str) -> usize {
    let mut summand = 0;
    for pattern in parse_input(contents) {
        match horizontal_almost_symmetry(&pattern) {
            Some(val) => {
                summand += val * 100;
            }
            None => {
                summand += horizontal_almost_symmetry(&transpose(&pattern)).unwrap();
            }
        }
    }
    summand
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d13.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(27502, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(31947, result);
    println!("part 2: {result}");
}
