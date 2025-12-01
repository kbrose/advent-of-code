use shared::Problem;

type Heights = [u8; 5];

fn parse_input(contents: &str) -> (Vec<Heights>, Vec<Heights>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic_str in contents.trim().split("\n\n") {
        let mut lines = schematic_str
            .split('\n')
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let mut is_key = false;
        if lines[0][0] == '.' {
            is_key = true;
            lines.reverse();
        }
        let mut heights: [u8; 5] = [0; 5];
        for (i, lines_i) in lines.iter().enumerate() {
            for (j, lines_ij) in lines_i.iter().enumerate() {
                if lines_ij == &'#' {
                    heights[j] = i as u8;
                }
            }
        }
        if is_key {
            keys.push(heights);
        } else {
            locks.push(heights);
        }
    }
    (locks, keys)
}

fn add_heights(h1: Heights, h2: Heights) -> Heights {
    let mut h3 = [0; 5];
    for (i, (a, b)) in h1.iter().zip(h2.iter()).enumerate() {
        h3[i] = a + b;
    }
    h3
}

fn compute_1(contents: &str) -> u64 {
    let (locks, keys) = parse_input(contents);
    let mut total = 0;
    for lock in locks {
        for key in keys.iter() {
            if add_heights(lock, *key).iter().all(|h| *h <= 5) {
                total += 1;
            }
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
    fn solve2(&self, _: &str) -> String {
        "no part 2".to_string()
    }
    fn expected1(&self) -> String {
        "2815".to_string()
    }
    fn expected2(&self) -> String {
        "no part 2".to_string()
    }
}
