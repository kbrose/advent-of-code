use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum XMAS {
    X,
    M,
    A,
    S,
}

fn parse_input(contents: &String) -> Vec<Vec<XMAS>> {
    contents
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'X' => XMAS::X,
                    'M' => XMAS::M,
                    'A' => XMAS::A,
                    'S' => XMAS::S,
                    _ => panic!("Unexpected character {c}!"),
                })
                .collect()
        })
        .collect()
}

enum Dir {
    Left,
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
}

fn new_i_j(i: usize, j: usize, dir: &Dir) -> (usize, usize) {
    match dir {
        Dir::Left => (i - 1, j),
        Dir::UpLeft => (i - 1, j + 1),
        Dir::Up => (i, j + 1),
        Dir::UpRight => (i + 1, j + 1),
        Dir::Right => (i + 1, j),
        Dir::DownRight => (i + 1, j - 1),
        Dir::Down => (i, j - 1),
        Dir::DownLeft => (i - 1, j - 1),
    }
}

fn safe_check(grid: &Vec<Vec<XMAS>>, i: usize, j: usize, expected: &XMAS) -> bool {
    grid.get(i)
        .map(|row| row.get(j))
        .is_some_and(|maybe_value| maybe_value.is_some_and(|value| value == expected))
}

fn probe(grid: &Vec<Vec<XMAS>>, mut i: usize, mut j: usize, dir: &Dir) -> bool {
    (i, j) = new_i_j(i, j, dir);
    if !safe_check(grid, i, j, &XMAS::M) {
        return false;
    }
    (i, j) = new_i_j(i, j, dir);
    if !safe_check(grid, i, j, &XMAS::A) {
        return false;
    }
    (i, j) = new_i_j(i, j, dir);
    if !safe_check(grid, i, j, &XMAS::S) {
        return false;
    }

    true
}

fn compute_1(contents: &String) -> u64 {
    let grid = parse_input(contents);
    let mut counter = 0;
    let dirs = vec![
        Dir::Left,
        Dir::UpLeft,
        Dir::Up,
        Dir::UpRight,
        Dir::Right,
        Dir::DownRight,
        Dir::Down,
        Dir::DownLeft,
    ];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == XMAS::X {
                for dir in dirs.iter() {
                    if probe(&grid, i, j, &dir) {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}

fn probe_2(grid: &Vec<Vec<XMAS>>, i: usize, j: usize) -> bool {
    if (safe_check(grid, i - 1, j - 1, &XMAS::M) && safe_check(grid, i + 1, j + 1, &XMAS::S))
        || (safe_check(grid, i - 1, j - 1, &XMAS::S) && safe_check(grid, i + 1, j + 1, &XMAS::M))
    {
        if (safe_check(grid, i - 1, j + 1, &XMAS::M) && safe_check(grid, i + 1, j - 1, &XMAS::S))
            || (safe_check(grid, i - 1, j + 1, &XMAS::S)
                && safe_check(grid, i + 1, j - 1, &XMAS::M))
        {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn compute_2(contents: &String) -> u64 {
    let grid = parse_input(contents);
    let mut counter = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == XMAS::A {
                if probe_2(&grid, i, j) {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d04.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(2654, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(1990, result);
    println!("part 2: {result}");
}
