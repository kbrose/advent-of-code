use shared::Problem;

fn parse_input(contents: &str) -> Vec<i64> {
    contents
        .trim()
        .lines()
        .map(|line| {
            let (direction, count) = line.split_at(1);
            count.parse::<i64>().unwrap() * { if direction == "L" { -1 } else { 1 } }
        })
        .collect()
}

fn compute_1(contents: &str) -> u64 {
    let rotations = parse_input(contents);
    let mut curr = 50;
    let mut counter = 0;
    for rotation in rotations {
        curr += rotation;
        curr = curr % 100; // x % 100 and x.rem_euclid(100) are equivalent here
        if curr == 0 {
            counter += 1;
        }
    }
    counter
}

fn compute_2(contents: &str) -> u64 {
    let rotations = parse_input(contents);
    let mut curr = 50;
    let mut counter = 0;
    for rotation in rotations {
        // To make the integer division by 100 work, we need to handle
        // negative rotations correctly.
        if curr != 0 && rotation < 0 {
            curr -= 100;
        }
        curr = curr + rotation;
        counter += curr.unsigned_abs() / 100;
        curr = curr.rem_euclid(100);
    }
    counter
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
        "1139".to_string()
    }
    fn expected2(&self) -> String {
        "6684".to_string()
    }
}
