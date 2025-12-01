use shared::Problem;

struct Race {
    time: u64,
    dist: u64,
}

fn parse_line_into_nums(line: &str) -> Vec<u64> {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn parse_input(contents: &str) -> Vec<Race> {
    let mut lines = contents.split('\n');
    let times = parse_line_into_nums(lines.next().unwrap());
    let dists = parse_line_into_nums(lines.next().unwrap());
    times
        .into_iter()
        .zip(dists)
        .map(|(time, dist)| Race { time, dist })
        .collect()
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn parse_input_2(contents: &str) -> Race {
    let mut lines = contents.split('\n');
    let time: u64 = remove_whitespace(lines.next().unwrap().split(':').nth(1).unwrap())
        .parse()
        .unwrap();
    let dist: u64 = remove_whitespace(lines.next().unwrap().split(':').nth(1).unwrap())
        .parse()
        .unwrap();
    Race { time, dist }
}

fn compute_1(contents: &str) -> u64 {
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

fn compute_2(contents: &str) -> u64 {
    let race = parse_input_2(contents);
    let t = race.time as f64;
    let d = race.dist as f64;
    let lower_bound = ((-t + (t.powf(2.0) - 4.0 * d).sqrt()) / (-2.0)).ceil();
    let upper_bound = ((-t - (t.powf(2.0) - 4.0 * d).sqrt()) / (-2.0)).floor();
    let possibilities = upper_bound - lower_bound;
    possibilities as u64 + 1
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
        "2449062".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "33149631".to_string()
    }
}
