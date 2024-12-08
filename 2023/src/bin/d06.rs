use std::fs;

struct Race {
    time: u64,
    dist: u64,
}

fn parse_line_into_nums(line: &str) -> Vec<u64> {
    line.split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn parse_input(contents: &String) -> Vec<Race> {
    let mut lines = contents.split('\n');
    let times = parse_line_into_nums(lines.next().unwrap());
    let dists = parse_line_into_nums(lines.next().unwrap());
    times
        .into_iter()
        .zip(dists.into_iter())
        .map(|(time, dist)| Race { time, dist })
        .collect()
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn parse_input_2(contents: String) -> Race {
    let mut lines = contents.split('\n');
    let time: u64 = remove_whitespace(lines.next().unwrap().split(':').nth(1).unwrap())
        .parse()
        .unwrap();
    let dist: u64 = remove_whitespace(lines.next().unwrap().split(':').nth(1).unwrap())
        .parse()
        .unwrap();
    Race { time, dist }
}

fn compute_1(contents: &String) -> u64 {
    // T = total time
    // h = held time
    // D = record distance traveled
    // Solve for number of non-negative integer h such that
    // (T - h) * h > D
    // (T - h) * h - D > 0
    // Th - h^2 - D > 0-
    // -h^2 + Th - D > 0
    // roots are (-T ± sqrt(T^2 - 4D)) / (-2)
    let races = parse_input(contents);
    let mut result = 1;
    for race in races {
        let t = race.time as f64;
        let d = race.dist as f64;
        let lower_bound = ((-t + (t.powf(2.0) - 4.0 * d).sqrt()) / (-2.0)).ceil();
        let upper_bound = ((-t - (t.powf(2.0) - 4.0 * d).sqrt()) / (-2.0)).floor();
        let possibilities = upper_bound - lower_bound;
        result *= possibilities as u64 + 1;
    }

    result
}

fn compute_2(contents: String) -> u64 {
    let race = parse_input_2(contents);
    let t = race.time as f64;
    let d = race.dist as f64;
    let lower_bound = ((-t + (t.powf(2.0) - 4.0 * d).sqrt()) / (-2.0)).ceil();
    let upper_bound = ((-t - (t.powf(2.0) - 4.0 * d).sqrt()) / (-2.0)).floor();
    let possibilities = upper_bound - lower_bound;
    possibilities as u64 + 1
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d06.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(2449062, result);
    println!("part 1: {result}");

    let result = compute_2(contents);
    assert_eq!(33149631, result);
    println!("part 2: {result}");
}
