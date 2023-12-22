use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
    Empty,
}

impl std::fmt::Display for Rock {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            Rock::Round => write!(f, "O"),
            Rock::Square => write!(f, "#"),
            Rock::Empty => write!(f, "."),
        }
    }
}

fn show(platform: &Platform) {
    for row in platform {
        for ele in row {
            print!("{ele}");
        }
        println!("");
    }
}

type Platform = Vec<Vec<Rock>>;

fn parse_input(contents: &String) -> Platform {
    contents
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Rock::Round,
                    '#' => Rock::Square,
                    '.' => Rock::Empty,
                    _ => panic!("unexpected character"),
                })
                .collect()
        })
        .collect()
}

fn tilt_north(platform: &mut Platform) {
    for col_index in 0..platform[0].len() {
        let mut next_location = 0;
        for row_index in 0..platform.len() {
            match platform[row_index][col_index] {
                Rock::Empty => {}
                Rock::Square => next_location = row_index + 1,
                Rock::Round => {
                    platform[row_index][col_index] = Rock::Empty;
                    platform[next_location][col_index] = Rock::Round;
                    next_location = next_location + 1;
                }
            }
        }
    }
}

fn compute_load(platform: &Platform) -> usize {
    let mut summand = 0;
    for (mut rows_from_south, row) in platform.iter().rev().enumerate() {
        rows_from_south += 1;
        for rock in row {
            match rock {
                Rock::Round => summand += rows_from_south,
                _ => {}
            }
        }
    }
    summand
}

fn compute_1(contents: &String) -> usize {
    let mut platform = parse_input(contents);
    // show(&platform);
    tilt_north(&mut platform);
    // println!("");
    // show(&platform);
    compute_load(&platform)
}

// fn compute_2(contents: &String) -> usize {
//     let mut platform = parse_input(contents);
//     tilt_north(&mut platform);
//     compute_load(&platform)
// }

fn main() {
    let contents =
        fs::read_to_string("inputs/d14.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(106648, result);
    println!("part 1: {result}");

    // let result = compute_2(&contents);
    // assert_eq!(31947, result);
    // println!("part 2: {result}");
}
