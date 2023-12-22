use std::fs;

fn get_number_1(line: &str) -> u64 {
    let mut first: char = '0';
    let mut second: char = '0';
    for c in line.chars() {
        if c.is_digit(10) {
            first = c;
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_digit(10) {
            second = c;
            break;
        }
    }
    format!("{first}{second}")
        .parse()
        .expect("Cannot parse number")
}

fn get_number_2(line: &str) -> u64 {
    let mut first: char = '0';
    let mut second: char = '0';

    let matches = [
        ("0", '0'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    for i in 0..line.len() {
        let mut done = false;
        for (query, assignment) in matches {
            if line[i..].starts_with(query) {
                first = assignment;
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }

    for i in (0..line.len()).rev() {
        let mut done = false;
        for (query, assignment) in matches {
            if line[i..].starts_with(query) {
                second = assignment;
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }

    format!("{first}{second}")
        .parse()
        .expect("Cannot parse number")
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d01.txt").expect("Should have been able to read the file");

    let result: u64 = contents.split("\n").map(get_number_1).sum();
    assert_eq!(55621, result);
    println!("part 1: {result}");

    let result: u64 = contents.split("\n").map(get_number_2).sum();
    assert_eq!(53592, result);
    println!("part 1: {result}");
}
