use std::fs;

type Pattern = Vec<Vec<bool>>;

fn transpose(pattern: &Pattern) -> Pattern {
    let mut new = vec![Vec::with_capacity(pattern.len()); pattern[0].len()];
    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            new[j].push(pattern[i][j]);
        }
    }
    new
}

fn parse_input(contents: &String) -> Vec<Pattern> {
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

fn compute_1(contents: &String) -> usize {
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

fn main() {
    let contents =
        fs::read_to_string("inputs/d13.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(27502, result);
    println!("part 1: {result}");

    // let result = compute_2(&contents);
    // assert_eq!(3476169006222, result);
    // println!("part 2: {result}");
}
