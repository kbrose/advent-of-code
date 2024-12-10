use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

const DIRS: [Dir; 4] = [Dir::N, Dir::S, Dir::E, Dir::W];

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Pipe((Dir, Dir)),
    Ground,
    Start,
}

impl Tile {
    fn new_pipe(d1: Dir, d2: Dir) -> Tile {
        match d1.cmp(&d2) {
            std::cmp::Ordering::Less => Tile::Pipe((d1, d2)),
            std::cmp::Ordering::Equal => {
                panic!("Pipe must have two distinct directions!");
            }
            std::cmp::Ordering::Greater => Tile::Pipe((d2, d1)),
        }
    }

    fn where_to(&self, dir_from: Dir) -> Dir {
        match self {
            Tile::Ground => Dir::N, // who cares
            Tile::Start => Dir::N,  // no but really, who cares
            Tile::Pipe((d1, d2)) => {
                if d1 == &dir_from {
                    *d2
                } else {
                    *d1
                }
            }
        }
    }

    fn solo_east_west_dir(&self) -> Option<Dir> {
        match self {
            Tile::Pipe((Dir::N, Dir::E)) => Some(Dir::E),
            Tile::Pipe((Dir::N, Dir::W)) => Some(Dir::W),
            Tile::Pipe((Dir::S, Dir::E)) => Some(Dir::E),
            Tile::Pipe((Dir::S, Dir::W)) => Some(Dir::W),
            _ => None,
        }
    }
}

type PipeGrid = Vec<Vec<Tile>>;

fn parse_input(contents: &str) -> (PipeGrid, (usize, usize)) {
    let mut s_i = 0;
    let mut s_j = 0;
    let pipes = contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '|' => Tile::new_pipe(Dir::N, Dir::S),
                    '-' => Tile::new_pipe(Dir::E, Dir::W),
                    'L' => Tile::new_pipe(Dir::N, Dir::E),
                    'J' => Tile::new_pipe(Dir::N, Dir::W),
                    '7' => Tile::new_pipe(Dir::S, Dir::W),
                    'F' => Tile::new_pipe(Dir::S, Dir::E),
                    '.' => Tile::Ground,
                    'S' => {
                        s_i = i;
                        s_j = j;
                        Tile::Start
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    (pipes, (s_i, s_j))
}

fn maybe_minus_one(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        n - 1
    }
}

fn update_i_j(i: usize, j: usize, dir_to: Dir) -> (usize, usize) {
    let new_i = match dir_to {
        Dir::N => i - 1,
        Dir::S => i + 1,
        _ => i,
    };
    let new_j = match dir_to {
        Dir::W => j - 1,
        Dir::E => j + 1,
        _ => j,
    };
    (new_i, new_j)
}

fn traverse_pipes(
    pipes: &PipeGrid,
    start_i: usize,
    start_j: usize,
    mut dir_to: Dir,
) -> (Vec<u64>, HashSet<(usize, usize)>) {
    let mut visited = HashSet::new();
    visited.insert((start_i, start_j));
    let mut dists = vec![]; // leave off initial zero so that reversing it aligns the two vectors
    let mut counter = 1;
    let (mut i, mut j) = update_i_j(start_i, start_j, dir_to);
    while (i, j) != (start_i, start_j) {
        visited.insert((i, j));
        dists.push(counter);
        counter += 1;
        let dir_from = dir_to.opposite();
        dir_to = pipes[i][j].where_to(dir_from);
        (i, j) = update_i_j(i, j, dir_to);
    }
    (dists, visited)
}

fn compute_1(contents: &str) -> u64 {
    let (pipes, (s_i, s_j)) = parse_input(contents);

    // Find the directions we can go to from the start.
    let mut dirs = vec![];
    for check_dir in DIRS {
        let pipe_to_check = match check_dir {
            Dir::N => pipes[maybe_minus_one(s_i)][s_j],
            Dir::S => pipes[std::cmp::min(s_i + 1, pipes.len())][s_j],
            Dir::E => pipes[s_i][std::cmp::min(s_j + 1, pipes[0].len())],
            Dir::W => pipes[s_i][maybe_minus_one(s_j)],
        };
        if DIRS
            .iter()
            .filter(|d| **d != check_dir.opposite())
            .map(|d| Tile::new_pipe(*d, check_dir.opposite()))
            .any(|p| p == pipe_to_check)
        {
            dirs.push(check_dir);
        }
    }
    assert!(dirs.len() == 2);

    let dists_1 = traverse_pipes(&pipes, s_i, s_j, dirs[0]);
    let dists_2 = traverse_pipes(&pipes, s_i, s_j, dirs[1]);

    *dists_1
        .0
        .iter()
        .zip(dists_2.0.iter().rev())
        .map(|(d1, d2)| std::cmp::min(d1, d2))
        .max()
        .unwrap()
}

fn compute_2(contents: String) -> u64 {
    let (mut pipes, (s_i, s_j)) = parse_input(&contents);
    // pipes.insert(0, vec![Pipe::Ground; pipes[0].len()]);
    // let s_i = s_i + 1;

    // Find the directions we can go to from the start.
    let mut dirs = vec![];
    for check_dir in DIRS {
        let pipe_to_check = match check_dir {
            Dir::N => pipes[maybe_minus_one(s_i)][s_j],
            Dir::S => pipes[std::cmp::min(s_i + 1, pipes.len())][s_j],
            Dir::E => pipes[s_i][std::cmp::min(s_j + 1, pipes[0].len())],
            Dir::W => pipes[s_i][maybe_minus_one(s_j)],
        };
        if DIRS
            .iter()
            .filter(|d| **d != check_dir.opposite())
            .map(|d| Tile::new_pipe(*d, check_dir.opposite()))
            .any(|p| p == pipe_to_check)
        {
            dirs.push(check_dir);
        }
    }
    assert!(dirs.len() == 2);

    let visited = traverse_pipes(&pipes, s_i, s_j, dirs[0]).1;

    pipes[s_i][s_j] = Tile::new_pipe(dirs[0], dirs[1]);

    let mut contained_count = 0;
    for i in 0..pipes.len() {
        for j in 0..pipes[0].len() {
            if !visited.contains(&(i, j)) {
                let mut odd_intersrection_count = false;
                let mut how_we_got_here: Option<Dir> = None;
                // cast a vertical ray upwards from the location
                // requires ugly bookkeeping to detect situations like
                //
                // --7.
                // ..|.
                // --J.
                //   X
                //
                // which shouldn't count as intersecting a pipe when casting the ray from X
                for (k, pipes_k) in pipes.iter().take(i).enumerate() {
                    if visited.contains(&(k, j)) {
                        let curr_pipes_east_west_dir = pipes_k[j].solo_east_west_dir();
                        match how_we_got_here {
                            Some(incoming_dir) => {
                                if let Some(d) = curr_pipes_east_west_dir {
                                    if d != incoming_dir {
                                        odd_intersrection_count = !odd_intersrection_count;
                                    }
                                    how_we_got_here = None;
                                }
                            }
                            None => match curr_pipes_east_west_dir {
                                Some(d) => {
                                    how_we_got_here = Some(d);
                                }
                                None => {
                                    odd_intersrection_count = !odd_intersrection_count;
                                }
                            },
                        }
                    } else {
                        how_we_got_here = None;
                    }
                }
                if odd_intersrection_count {
                    contained_count += 1;
                }
            }
        }
    }
    contained_count
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d10.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(6903, result);
    println!("part 1: {result}");

    let result = compute_2(contents);
    assert_eq!(265, result);
    println!("part 2: {result}");
}
