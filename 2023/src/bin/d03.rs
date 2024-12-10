use std::cmp;
use std::fs;

fn maybe_minus_one(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        n - 1
    }
}

fn compute_1(contents: &str) -> u64 {
    let mut summand: u64 = 0;
    let lines: Vec<&str> = contents.split('\n').filter(|s| !s.is_empty()).collect();
    let num_lines = lines.len();
    for (line_index, line) in lines.iter().enumerate() {
        let mut valid_num = false;
        let mut num: u64 = 0;
        for (char_index, c) in line.chars().chain(".".chars()).enumerate() {
            if "0123456789".contains(c) {
                num *= 10;
                num += c.to_digit(10).unwrap() as u64;
                for query_line in lines
                    .iter()
                    .take(cmp::min(line_index + 2, num_lines))
                    .skip(maybe_minus_one(line_index))
                {
                    let query_string = &query_line
                        [maybe_minus_one(char_index)..cmp::min(char_index + 2, query_line.len())];
                    if query_string.chars().any(|c| !"0123456789.".contains(c)) {
                        valid_num = true;
                    }
                }
            } else {
                if num > 0 && valid_num {
                    summand += num;
                }
                valid_num = false;
                num = 0;
            }
        }
    }
    summand
}

#[derive(Debug)]
struct Gear {
    line_index: usize,
    char_index: usize,
}
#[derive(Debug)]
struct Number {
    line_index: usize,
    char_start: usize,
    char_end: usize,
    num: u64,
}

fn num_matches_gear(gear: &Gear, num: &Number) -> bool {
    if maybe_minus_one(gear.line_index) <= num.line_index
        && num.line_index <= gear.line_index + 1
        && maybe_minus_one(num.char_start) <= gear.char_index
        && gear.char_index <= num.char_end + 1
    {
        return true;
    }
    false
}

fn compute_2(contents: String) -> u64 {
    let mut potential_gears: Vec<Gear> = vec![];
    let mut numbers: Vec<Number> = Vec::new(); // <(usize, usize, u64)> = vec![];
    for (line_index, line) in contents.split('\n').filter(|s| !s.is_empty()).enumerate() {
        let mut num = 0;
        let mut start = usize::MAX;
        let mut stop = 0;
        for (char_index, c) in line.chars().chain(".".chars()).enumerate() {
            if "0123456789".contains(c) {
                num *= 10;
                num += c.to_digit(10).unwrap() as u64;
                start = cmp::min(start, char_index);
                stop = char_index;
            } else {
                if num > 0 {
                    numbers.push(Number {
                        line_index,
                        char_start: start,
                        char_end: stop,
                        num,
                    });
                }
                num = 0;
                start = usize::MAX;
                stop = 0;
                if c == '*' {
                    potential_gears.push(Gear {
                        line_index,
                        char_index,
                    })
                }
            }
        }
    }
    let mut summand = 0;
    for gear in potential_gears {
        let matches: Vec<&Number> = numbers
            .iter()
            .filter(|n| num_matches_gear(&gear, n))
            .collect();
        if matches.len() == 2 {
            summand += matches[0].num * matches[1].num;
        }
    }
    summand
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d03.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(525119, result);
    println!("part 1: {result}");

    let result = compute_2(contents);
    assert_eq!(76504829, result);
    println!("part 2: {result}");
}
