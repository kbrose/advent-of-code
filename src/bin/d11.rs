use std::{collections::HashSet, fs};

fn parse_input(contents: &String) -> Vec<(usize, usize)> {
    let mut locations = vec![];
    for (i, line) in contents.split('\n').filter(|l| l.len() > 0).enumerate() {
        for j in line
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'#')
            .map(|(j, _)| j)
        {
            locations.push((i, j));
        }
    }
    let occupied_rows: Vec<usize> = locations
        .iter()
        .map(|(i, _)| *i)
        .collect::<HashSet<usize>>()
        .into_iter()
        .collect();
    let occupied_cols: Vec<usize> = locations
        .iter()
        .map(|(_, j)| *j)
        .collect::<HashSet<usize>>()
        .into_iter()
        .collect();
    locations = locations
        .into_iter()
        .map(|(i, j)| {
            let unoccupied_row_count = i - occupied_rows.iter().filter(|r| r < &&i).count();
            let unoccupied_col_count = j - occupied_cols.iter().filter(|r| r < &&j).count();
            (i + unoccupied_row_count, j + unoccupied_col_count)
        })
        .collect();
    locations
}

fn absolute_difference(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn compute_1(contents: &String) -> usize {
    let mut summand = 0;
    let locations = parse_input(contents);
    for (l1_index, l1) in locations.iter().enumerate() {
        for l2 in locations.iter().skip(l1_index + 1) {
            summand += absolute_difference(l1.0, l2.0) + absolute_difference(l1.1, l2.1);
        }
    }
    summand
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d11.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(9693756, result);
    println!("part 1: {result}");

    // let result = compute_2(contents);
    // assert_eq!(265, result);
    // println!("part 2: {result}");
}
