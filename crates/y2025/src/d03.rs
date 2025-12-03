use shared::Problem;

fn parse_input(contents: &str) -> Vec<Vec<u64>> {
    contents
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect()
}

/// argmax returns the index of the highest value.
/// If there is a tie, the FIRST index is returned.
/// (That is necessary for correct puzzle output.)
fn argmax(v: &[u64]) -> usize {
    assert!(v.len() > 0);
    let mut i = 0;
    let mut curr_max = 0;
    for (j, n) in v.iter().enumerate() {
        if *n > curr_max {
            i = j;
            curr_max = *n;
        }
    }
    i
}

fn compute_1(contents: &str) -> u64 {
    let battery_banks = parse_input(contents);
    let mut total = 0;
    for bank in battery_banks {
        let i = argmax(&bank[..bank.len() - 1]);
        let joltage = bank[i] * 10 + bank[(i + 1)..].iter().max().unwrap();
        total += joltage;
    }
    total
}

fn compute_2(contents: &str) -> u64 {
    let battery_banks = parse_input(contents);
    let mut total = 0;
    for bank in battery_banks {
        let mut joltage = 0;
        let mut starting_index = 0;
        for digit in (0..12).rev() {
            starting_index = starting_index + argmax(&bank[starting_index..bank.len() - digit]);
            joltage += bank[starting_index] * 10_u64.pow(digit as u32);
            starting_index += 1;
        }
        total += joltage;
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
        "17166".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "169077317650774".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_argmax() {
        assert_eq!(argmax(&[1, 2, 3]), 2);
        assert_eq!(argmax(&[3, 2, 1]), 0);
        assert_eq!(argmax(&[1, 3, 2]), 1);
        assert_eq!(argmax(&[1, 1, 0]), 0);
        assert_eq!(argmax(&[0]), 0);
    }
}
