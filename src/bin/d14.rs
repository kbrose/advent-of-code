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

// fn show(platform: &Platform) {
//     for row in platform {
//         for ele in row {
//             print!("{ele}");
//         }
//         println!("");
//     }
// }

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

fn tilt_north_south(platform: &mut Platform, is_north: bool) {
    let num_rows = platform.len();
    let num_cols = platform[0].len();
    for col_index in 0..num_cols {
        let mut next_location = if is_north { 0 } else { num_rows - 1 };
        for mut row_index in 0..num_rows {
            if !is_north {
                row_index = num_rows - row_index - 1;
            }
            match platform[row_index][col_index] {
                Rock::Empty => {}
                Rock::Square => {
                    next_location = if is_north {
                        row_index + 1
                    } else {
                        row_index - 1
                    }
                }
                Rock::Round => {
                    platform[row_index][col_index] = Rock::Empty;
                    platform[next_location][col_index] = Rock::Round;
                    next_location = if is_north {
                        next_location + 1
                    } else {
                        next_location - 1
                    };
                }
            }
        }
    }
}

fn tilt_east_west(platform: &mut Platform, is_west: bool) {
    let num_rows = platform.len();
    let num_cols = platform[0].len();
    for row_index in 0..num_rows {
        let mut next_location = if is_west { 0 } else { num_cols - 1 };
        for mut col_index in 0..num_cols {
            if !is_west {
                col_index = num_cols - col_index - 1;
            }
            match platform[row_index][col_index] {
                Rock::Empty => {}
                Rock::Square => {
                    next_location = if is_west {
                        col_index + 1
                    } else {
                        col_index - 1
                    }
                }
                Rock::Round => {
                    platform[row_index][col_index] = Rock::Empty;
                    platform[row_index][next_location] = Rock::Round;
                    next_location = if is_west {
                        next_location + 1
                    } else {
                        next_location - 1
                    };
                }
            }
        }
    }
}

fn get_round_rock_locations(platform: &Platform) -> std::collections::HashSet<(usize, usize)> {
    let mut locations = std::collections::HashSet::new();
    for (i, row) in platform.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if element == &Rock::Round {
                locations.insert((i, j));
            }
        }
    }
    locations
}

fn compute_2(contents: &String) -> usize {
    let mut platform = parse_input(contents);
    let mut locations = Vec::new();
    let mut loads = Vec::new();
    for iteration_count in 0..10000000 {
        tilt_north_south(&mut platform, true);
        tilt_east_west(&mut platform, true);
        tilt_north_south(&mut platform, false);
        tilt_east_west(&mut platform, false);
        let curr_locations = get_round_rock_locations(&platform);
        loads.push(compute_load(&platform));
        if locations.contains(&curr_locations) {
            let matching_iter_count = locations
                .iter()
                .enumerate()
                .filter(|(_, l)| **l == curr_locations)
                .next()
                .unwrap()
                .0;
            let cycle_len = iteration_count - matching_iter_count;
            let cycle_index = (1000000000 - matching_iter_count - 1) % cycle_len;
            return loads[matching_iter_count + cycle_index];
        }
        locations.push(curr_locations);
    }
    compute_load(&platform)
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d14.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(106648, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(87700, result);
    println!("part 2: {result}");
}
