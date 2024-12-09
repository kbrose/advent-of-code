use std::fs;

fn parse_input(contents: &String) -> Vec<Vec<i64>> {
    contents
        .trim()
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(levels: &Vec<i64>) -> bool {
    assert!(levels.len() >= 2);
    let delta1 = levels[1] - levels[0];
    if delta1.abs() < 0 || delta1.abs() > 3 {
        return false;
    }
    let sign = delta1.signum();
    for i in 2..levels.len() {
        let delta = levels[i] - levels[i - 1];
        if delta.signum() != sign {
            return false;
        } else {
            if delta.abs() > 3 {
                // signnum() is 0 if delta is 0, so only need to check > 3
                return false;
            }
        }
    }
    true
}

fn compute_1(contents: &String) -> usize {
    let reports = parse_input(contents);
    reports.iter().filter(|levels| is_safe(levels)).count()
}

fn is_safe_2(levels: &Vec<i64>) -> bool {
    if is_safe(levels) {
        return true;
    } else {
        // Clearly suboptimal...
        for i in 0..levels.len() {
            let mut levels2 = levels.clone();
            levels2.remove(i);
            if is_safe(&levels2) {
                return true;
            }
        }
    }
    false
}

fn compute_2(contents: &String) -> usize {
    let reports = parse_input(contents);
    reports.iter().filter(|levels| is_safe_2(levels)).count()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d02.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(332, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(398, result);
    println!("part 2: {result}");
}
