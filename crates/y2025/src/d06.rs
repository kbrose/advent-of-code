use shared::Problem;

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Mul,
    Sum,
}

fn parse_input(contents: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();
    for mut line in contents.trim().lines() {
        line = line.trim();
        if line.chars().next().unwrap().is_digit(10) {
            problems.push(
                line.split_ascii_whitespace()
                    .map(|num_str| num_str.parse::<u64>().unwrap())
                    .collect(),
            );
        } else {
            operations.extend(line.split_ascii_whitespace().map(|c| match c {
                "+" => Operation::Sum,
                "*" => Operation::Mul,
                _ => panic!("Unknown operation {c}"),
            }))
        }
    }
    let mut transposed_problems: Vec<Vec<u64>> = vec![vec![]; problems[0].len()];
    for i in 0..transposed_problems.len() {
        for problem in problems.iter() {
            transposed_problems[i].push(problem[i]);
        }
    }
    (transposed_problems, operations)
}

fn compute_1(contents: &str) -> u64 {
    let (problems, operations) = parse_input(contents);
    let mut total = 0;
    for (problem, operation) in problems.into_iter().zip(operations.into_iter()) {
        match operation {
            Operation::Mul => total += problem.into_iter().reduce(|a, b| a * b).unwrap(),
            Operation::Sum => total += problem.into_iter().reduce(|a, b| a + b).unwrap(),
        }
    }
    total
}

fn parse_input_2(contents: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let mut problems: Vec<Vec<char>> = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();
    for line in contents.trim().lines() {
        if line.trim().chars().next().unwrap().is_digit(10) {
            problems.push(line.chars().collect());
        } else {
            operations.extend(line.trim().split_ascii_whitespace().map(|c| match c {
                "+" => Operation::Sum,
                "*" => Operation::Mul,
                _ => panic!("Unknown operation {c}"),
            }))
        }
    }
    let mut transposed_problems: Vec<Vec<char>> = vec![vec![]; problems[0].len()];
    assert!(problems.iter().all(|p| p.len() == problems[0].len()));
    for i in 0..transposed_problems.len() {
        for problem in problems.iter() {
            transposed_problems[i].push(problem[i]);
        }
    }
    let problems: String = transposed_problems
        .into_iter()
        .map(|line| {
            line.into_iter()
                .filter(|c| c != &'\n')
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
                .trim_end()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n");
    let mut final_problems = vec![];
    for mut line in problems.split("\n\n") {
        line = line.trim();
        final_problems.push(
            line.split_ascii_whitespace()
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect(),
        )
    }
    (final_problems, operations)
}

fn compute_2(contents: &str) -> u64 {
    let (problems, operations) = parse_input_2(contents);
    let mut total = 0;
    for (problem, operation) in problems.into_iter().zip(operations.into_iter()) {
        match operation {
            Operation::Mul => total += problem.into_iter().reduce(|a, b| a * b).unwrap(),
            Operation::Sum => total += problem.into_iter().reduce(|a, b| a + b).unwrap(),
        }
    }
    total
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
        "4580995422905".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "10875057285868".to_string()
    }
}
