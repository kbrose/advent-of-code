use shared::Problem;

fn parse_input(contents: &str) -> Vec<Vec<i64>> {
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

fn is_safe(levels: &[i64]) -> bool {
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
        } else if delta.abs() > 3 {
            // signnum() is 0 if delta is 0, so only need to check > 3
            return false;
        }
    }
    true
}

fn compute_1(contents: &str) -> usize {
    let reports = parse_input(contents);
    reports.iter().filter(|levels| is_safe(levels)).count()
}

fn is_safe_2(levels: &[i64]) -> bool {
    if is_safe(levels) {
        return true;
    } else {
        // Clearly suboptimal...
        for i in 0..levels.len() {
            let mut levels2 = levels.to_owned();
            levels2.remove(i);
            if is_safe(&levels2) {
                return true;
            }
        }
    }
    false
}

fn compute_2(contents: &str) -> usize {
    let reports = parse_input(contents);
    reports.iter().filter(|levels| is_safe_2(levels)).count()
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
        "332".to_string()
    }
    fn expected2(&self) -> String {
        "398".to_string()
    }
}
