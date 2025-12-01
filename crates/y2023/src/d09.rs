use shared::Problem;

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

fn compute_2(contents: &str) -> i64 {
    let sequences = parse_input(&contents);

    sequences.iter().map(find_prev).sum()
}

pub(crate) struct Day {}

impl Problem for Day {
    fn source_code_file(&self) -> String {
        file!().to_string()
    }
    fn solve1(&self, contents: &str) -> String {
        format!("{}", compute_1(contents))
    }
    fn expected1(&self) -> String {
        "1479011877".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "973".to_string()
    }
}
