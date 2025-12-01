use shared::Problem;

#[derive(Debug, PartialEq, Eq)]
enum Xmas {
    X,
    M,
    A,
    S,
}

fn parse_input(contents: &str) -> Vec<Vec<Xmas>> {
    contents
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'X' => Xmas::X,
                    'M' => Xmas::M,
                    'A' => Xmas::A,
                    'S' => Xmas::S,
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

fn safe_check(grid: &[Vec<Xmas>], i: usize, j: usize, expected: &Xmas) -> bool {
    grid.get(i)
        .map(|row| row.get(j))
        .is_some_and(|maybe_value| maybe_value.is_some_and(|value| value == expected))
}

fn probe(grid: &[Vec<Xmas>], mut i: usize, mut j: usize, dir: &Dir) -> bool {
    (i, j) = new_i_j(i, j, dir);
    if !safe_check(grid, i, j, &Xmas::M) {
        return false;
    }
    (i, j) = new_i_j(i, j, dir);
    if !safe_check(grid, i, j, &Xmas::A) {
        return false;
    }
    (i, j) = new_i_j(i, j, dir);
    if !safe_check(grid, i, j, &Xmas::S) {
        return false;
    }

    true
}

fn compute_1(contents: &str) -> u64 {
    let grid = parse_input(contents);
    let mut counter = 0;
    let dirs = [
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
            if grid[i][j] == Xmas::X {
                for dir in dirs.iter() {
                    if probe(&grid, i, j, dir) {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}

fn probe_2(grid: &[Vec<Xmas>], i: usize, j: usize) -> bool {
    if (safe_check(grid, i - 1, j - 1, &Xmas::M) && safe_check(grid, i + 1, j + 1, &Xmas::S))
        || (safe_check(grid, i - 1, j - 1, &Xmas::S) && safe_check(grid, i + 1, j + 1, &Xmas::M))
    {
        (safe_check(grid, i - 1, j + 1, &Xmas::M) && safe_check(grid, i + 1, j - 1, &Xmas::S))
            || (safe_check(grid, i - 1, j + 1, &Xmas::S)
                && safe_check(grid, i + 1, j - 1, &Xmas::M))
    } else {
        false
    }
}

fn compute_2(contents: &str) -> u64 {
    let grid = parse_input(contents);
    let mut counter = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Xmas::A && probe_2(&grid, i, j) {
                counter += 1;
            }
        }
    }
    counter
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
        "2654".to_string()
    }
    fn expected2(&self) -> String {
        "1990".to_string()
    }
}
