use shared::Problem;

use std::collections::HashSet;

fn parse_input(contents: &str, expansion_factor: usize) -> Vec<(usize, usize)> {
    let mut locations = vec![];
    for (i, line) in contents.split('\n').filter(|l| !l.is_empty()).enumerate() {
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
            (
                i + unoccupied_row_count * (expansion_factor - 1),
                j + unoccupied_col_count * (expansion_factor - 1),
            )
        })
        .collect();
    locations
}

fn absolute_difference(a: usize, b: usize) -> usize {
    if a < b { b - a } else { a - b }
}

fn compute(contents: &str, expansion_factor: usize) -> usize {
    let mut summand = 0;
    let locations = parse_input(contents, expansion_factor);
    for (l1_index, l1) in locations.iter().enumerate() {
        for l2 in locations.iter().skip(l1_index + 1) {
            summand += absolute_difference(l1.0, l2.0) + absolute_difference(l1.1, l2.1);
        }
    }
    summand
}

pub(crate) struct Day {}

impl Problem for Day {
    fn source_code_file(&self) -> String {
        file!().to_string()
    }
    fn solve1(&self, contents: &str) -> String {
        format!("{}", compute(contents, 2))
    }
    fn expected1(&self) -> String {
        "9693756".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute(contents, 1_000_000))
    }
    fn expected2(&self) -> String {
        "717878258016".to_string()
    }
}
