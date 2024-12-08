use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn left_turn(&self) -> Dir {
        match self {
            Dir::U => Dir::L,
            Dir::D => Dir::R,
            Dir::L => Dir::D,
            Dir::R => Dir::U,
        }
    }
    fn right_turn(&self) -> Dir {
        match self {
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::L => Dir::U,
            Dir::R => Dir::D,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Vector {
    dir: Dir,
    count: u16,
}

fn try_move(
    dir: Dir,
    row: usize,
    col: usize,
    num_rows: usize,
    num_cols: usize,
) -> Option<(usize, usize)> {
    match dir {
        Dir::U => {
            if row == 0 {
                None
            } else {
                Some((row - 1, col))
            }
        }
        Dir::D => {
            if row == num_rows - 1 {
                None
            } else {
                Some((row + 1, col))
            }
        }
        Dir::L => {
            if col == 0 {
                None
            } else {
                Some((row, col - 1))
            }
        }
        Dir::R => {
            if col == num_cols - 1 {
                None
            } else {
                Some((row, col + 1))
            }
        }
    }
}

fn parse_input(contents: &String) -> Vec<Vec<u64>> {
    contents
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn compute_1(contents: &String) -> Option<u64> {
    let city = parse_input(contents);
    let num_rows = city.len();
    let num_cols = city[0].len();
    let mut visited: HashMap<(usize, usize, Vector), u64> = HashMap::new();
    let mut todo: BinaryHeap<Reverse<(u64, usize, usize, Vector)>> = BinaryHeap::new();
    todo.push(Reverse((
        city[0][1],
        0,
        1,
        Vector {
            dir: Dir::R,
            count: 1,
        },
    )));
    todo.push(Reverse((
        city[1][0],
        1,
        0,
        Vector {
            dir: Dir::D,
            count: 1,
        },
    )));
    while todo.len() > 0 {
        let Reverse((cost, row, col, vec)) = todo.pop().unwrap();
        if (row, col) == (num_rows - 1, num_cols - 1) {
            return Some(cost);
        }
        if visited.contains_key(&(row, col, vec)) {
            continue;
        }
        visited.insert((row, col, vec), cost);

        let dir = vec.dir.left_turn();
        if let Some((new_row, new_col)) = try_move(dir, row, col, num_rows, num_cols) {
            let new_vec = Vector { dir, count: 1 };
            if !visited.contains_key(&(new_row, new_col, new_vec)) {
                todo.push(Reverse((
                    cost + city[new_row][new_col],
                    new_row,
                    new_col,
                    new_vec,
                )))
            }
        }

        let dir = vec.dir.right_turn();
        if let Some((new_row, new_col)) = try_move(dir, row, col, num_rows, num_cols) {
            let new_vec = Vector { dir, count: 1 };
            if !visited.contains_key(&(new_row, new_col, new_vec)) {
                todo.push(Reverse((
                    cost + city[new_row][new_col],
                    new_row,
                    new_col,
                    new_vec,
                )))
            }
        }

        if vec.count < 3 {
            let dir = vec.dir;
            if let Some((new_row, new_col)) = try_move(dir, row, col, num_rows, num_cols) {
                let new_vec = Vector {
                    dir,
                    count: vec.count + 1,
                };
                if !visited.contains_key(&(new_row, new_col, new_vec)) {
                    todo.push(Reverse((
                        cost + city[new_row][new_col],
                        new_row,
                        new_col,
                        new_vec,
                    )))
                }
            }
        }
    }
    None
}

fn compute_2(contents: &String) -> Option<u64> {
    let city = parse_input(contents);
    let num_rows = city.len();
    let num_cols = city[0].len();
    let mut visited: HashMap<(usize, usize, Vector), u64> = HashMap::new();
    let mut todo: BinaryHeap<Reverse<(u64, usize, usize, Vector)>> = BinaryHeap::new();
    todo.push(Reverse((
        city[0][1],
        0,
        1,
        Vector {
            dir: Dir::R,
            count: 1,
        },
    )));
    todo.push(Reverse((
        city[1][0],
        1,
        0,
        Vector {
            dir: Dir::D,
            count: 1,
        },
    )));
    while todo.len() > 0 {
        let Reverse((cost, row, col, vec)) = todo.pop().unwrap();
        if (row, col) == (num_rows - 1, num_cols - 1) && vec.count >= 4 {
            return Some(cost);
        }
        if visited.contains_key(&(row, col, vec)) {
            continue;
        }
        visited.insert((row, col, vec), cost);

        if vec.count >= 4 {
            let dir = vec.dir.left_turn();
            if let Some((new_row, new_col)) = try_move(dir, row, col, num_rows, num_cols) {
                let new_vec = Vector { dir, count: 1 };
                if !visited.contains_key(&(new_row, new_col, new_vec)) {
                    todo.push(Reverse((
                        cost + city[new_row][new_col],
                        new_row,
                        new_col,
                        new_vec,
                    )))
                }
            }

            let dir = vec.dir.right_turn();
            if let Some((new_row, new_col)) = try_move(dir, row, col, num_rows, num_cols) {
                let new_vec = Vector { dir, count: 1 };
                if !visited.contains_key(&(new_row, new_col, new_vec)) {
                    todo.push(Reverse((
                        cost + city[new_row][new_col],
                        new_row,
                        new_col,
                        new_vec,
                    )))
                }
            }
        }

        if vec.count < 10 {
            let dir = vec.dir;
            if let Some((new_row, new_col)) = try_move(dir, row, col, num_rows, num_cols) {
                let new_vec = Vector {
                    dir,
                    count: vec.count + 1,
                };
                if !visited.contains_key(&(new_row, new_col, new_vec)) {
                    todo.push(Reverse((
                        cost + city[new_row][new_col],
                        new_row,
                        new_col,
                        new_vec,
                    )))
                }
            }
        }
    }
    for count in 0..11 {
        for dir in [Dir::D, Dir::R, Dir::U, Dir::L] {
            let key = (num_rows, num_cols, Vector { dir, count });
            if visited.contains_key(&key) {
                let x = visited[&key];
                println!("{x}");
            }
        }
    }
    None
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d17.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents).expect("Unable to find solution!");
    assert_eq!(859, result);
    println!("part 1: {result}");

    let result = compute_2(&contents).expect("Unable to find solution for part 2!");
    assert_eq!(1027, result);
    println!("part 2: {result}");
}
