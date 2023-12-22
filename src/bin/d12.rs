use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spring {
    Works,
    Broke,
    Dunno,
}

fn parse_input(contents: &String) -> Vec<(Vec<Spring>, Vec<u64>)> {
    contents
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut split = line.split_whitespace();
            let springs = split
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Spring::Works,
                    '#' => Spring::Broke,
                    '?' => Spring::Dunno,
                    _ => panic!(),
                })
                .collect();
            let contiguous_counts = split
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (springs, contiguous_counts)
        })
        .collect()
}

fn is_valid(springs: &Vec<Spring>, contiguous_counts: &Vec<u64>) -> Option<bool> {
    let mut observed_counts = vec![];
    let mut in_streak = false;
    for s in springs {
        if !in_streak && observed_counts.len() > 0 {
            // short circuit test
            if observed_counts.len() > contiguous_counts.len() {
                return Some(false);
            }
            if observed_counts.last().unwrap() != &contiguous_counts[observed_counts.len() - 1] {
                return Some(false);
            }
        }
        match s {
            Spring::Dunno => return None,
            Spring::Works => {
                in_streak = false;
            }
            Spring::Broke => {
                if in_streak {
                    *observed_counts.last_mut().unwrap() += 1;
                } else {
                    observed_counts.push(1);
                    in_streak = true;
                }
            }
        }
    }
    Some(contiguous_counts == &observed_counts)
}

fn count_arrangements(springs: &Vec<Spring>, contiguous_counts: &Vec<u64>, i: usize) -> u64 {
    match is_valid(springs, contiguous_counts) {
        Some(true) => 1,
        Some(false) => 0,
        None => match springs[i] {
            Spring::Dunno => {
                let mut springs_1 = springs.clone();
                springs_1[i] = Spring::Broke;
                let mut springs_2 = springs.clone();
                springs_2[i] = Spring::Works;
                count_arrangements(&springs_1, contiguous_counts, i + 1)
                    + count_arrangements(&springs_2, contiguous_counts, i + 1)
            }
            _ => count_arrangements(springs, contiguous_counts, i + 1),
        },
    }
}

fn compute_1(contents: &String) -> u64 {
    parse_input(contents)
        .iter()
        .map(|(a, b)| count_arrangements(a, b, 0))
        .sum()
}

fn parse_input_2(contents: &String, n: usize) -> Vec<(Vec<Spring>, Vec<usize>)> {
    contents
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut split = line.split_whitespace();
            let springs_string = split.next().unwrap();
            let springs_string = format!("{springs_string}?");
            let springs = "."
                .chars()
                .chain(
                    springs_string
                        .chars()
                        .cycle()
                        .take(springs_string.len() * n - 1),
                )
                .chain(".".chars())
                .map(|c| match c {
                    '.' => Spring::Works,
                    '#' => Spring::Broke,
                    '?' => Spring::Dunno,
                    _ => panic!("Unexpected character in spring string"),
                })
                .collect();

            let counts_strings: Vec<_> = split.next().unwrap().trim().split(',').collect();
            let contiguous_counts = counts_strings
                .iter()
                .cycle()
                .take(counts_strings.len() * n)
                .map(|s| s.parse().unwrap())
                .collect();
            (springs, contiguous_counts)
        })
        .collect()
}

fn is_compatible(reference: &[Spring], candidate: &[Spring]) -> bool {
    if reference.len() != candidate.len() {
        false
    } else {
        reference
            .iter()
            .zip(candidate.iter())
            .all(|(r, c)| r == c || r == &Spring::Dunno)
    }
}

// Example solution matrix to ?????#???? 2,1,1
// A "." is prepended and appended to every springs input due to assumptions of the algorithm.
// These do not change the answer.
//
// Let springs and contiguous_counts be the algorithm inputs.
//
// matrix[i][j] is a tuple (n, m) where:
// n is the number of possible solutions given springs[..j+1] and contiguous_counts[..i+1]
// m is max({last index into springs whose value was fixed | all solutions up to i,j})
//
// Every # we encounter resets the tuple to (0, 0). TBH I don't remember why. I was in a sort
// of fugue state after trying for a long time to create this dynamic programming solution.
// I'm sure it was well reasoned at the time.
//
//   0        1        2        3        4        5        6        7        8        9        10       11
//   .        ?        ?        ?        ?        ?        #        ?        ?        ?        ?        .
// 2 (0, 0)   (0, 0)   (0, 0)   (1, 3)   (2, 4)   (3, 5)   (0, 0)   (1, 7)   (2, 8)   (2, 8)   (2, 8)   (2, 8)
// 1 (0, 0)   (0, 0)   (0, 0)   (0, 0)   (0, 0)   (1, 5)   (0, 0)   (3, 7)   (3, 7)   (4, 9)   (6, 10)  (8, 11)
// 1 (0, 0)   (0, 0)   (0, 0)   (0, 0)   (0, 0)   (0, 0)   (0, 0)   (1, 7)   (1, 7)   (4, 9)   (7, 10)  (11, 11)
fn count_arrangements_2(springs: &Vec<Spring>, contiguous_counts: &Vec<usize>) -> u64 {
    let mut matrix: Vec<Vec<(u64, usize)>> = vec![];
    for _ in 0..(contiguous_counts.len() + 1) {
        matrix.push(vec![(0, 0); springs.len()]);
    }
    for (mut i, c) in contiguous_counts.iter().enumerate() {
        i += 1;
        let mut j_start = contiguous_counts.iter().take(i).sum::<usize>() + i;
        if i > 1 {
            match matrix[i - 1]
                .iter()
                .map(|(counts, _)| *counts > 0)
                .enumerate()
                .filter(|(_, valid)| *valid)
                .next()
            {
                Some((index, _)) => j_start = std::cmp::max(index + c + 1, j_start),
                None => return 0,
            }
        }
        for j in j_start..springs.len() {
            if springs[j] == Spring::Broke {
                continue;
            }
            matrix[i][j] = matrix[i][j - 1];
            // lookback_index is the index of the previous row whose solutions
            // can possibly feed into the current location's
            let lookback_index = j.checked_sub(*c).unwrap().checked_sub(1).unwrap();
            let start_index = matrix[i - 1][lookback_index].1;
            let mut candidate_spring_seq = vec![Spring::Works; j - c - start_index];
            for _ in 0..*c {
                candidate_spring_seq.push(Spring::Broke);
            }
            candidate_spring_seq.push(Spring::Works);
            if is_compatible(&springs[start_index..j + 1], &candidate_spring_seq) {
                matrix[i][j] = (
                    matrix[i][j].0 + std::cmp::max(matrix[i - 1][lookback_index].0, 1),
                    j,
                );
            }
        }
    }
    matrix.last().unwrap().last().unwrap().0
}

fn compute_2(contents: &String) -> u64 {
    parse_input_2(contents, 5)
        .iter()
        .map(|(a, b)| count_arrangements_2(&a, &b))
        .sum()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d12.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(7007, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(3476169006222, result);
    println!("part 2: {result}");
}
