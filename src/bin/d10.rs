use std::fs;

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

#[derive(PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Pipe((Dir, Dir)),
    Ground,
    Start,
}

impl Pipe {
    fn new_pipe(d1: Dir, d2: Dir) -> Pipe {
        if d1 == d2 {
            panic!("Pipe must have two distinct directions!");
        } else if d1 < d2 {
            Pipe::Pipe((d1, d2))
        } else {
            Pipe::Pipe((d2, d1))
        }
    }

    fn where_to(&self, dir_from: Dir) -> Dir {
        match self {
            Pipe::Ground => Dir::N, // who cares
            Pipe::Start => Dir::N,  // no but really, who cares
            Pipe::Pipe((d1, d2)) => {
                if d1 == &dir_from {
                    *d2
                } else {
                    *d1
                }
            }
        }
    }
}

type PipeGrid = Vec<Vec<Pipe>>;

fn parse_input(contents: &String) -> (PipeGrid, (usize, usize)) {
    let mut s_i = 0;
    let mut s_j = 0;
    let pipes = contents
        .split('\n')
        .filter(|l| l.len() > 0)
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '|' => Pipe::new_pipe(Dir::N, Dir::S),
                    '-' => Pipe::new_pipe(Dir::E, Dir::W),
                    'L' => Pipe::new_pipe(Dir::N, Dir::E),
                    'J' => Pipe::new_pipe(Dir::N, Dir::W),
                    '7' => Pipe::new_pipe(Dir::S, Dir::W),
                    'F' => Pipe::new_pipe(Dir::S, Dir::E),
                    '.' => Pipe::Ground,
                    'S' => {
                        s_i = i;
                        s_j = j;
                        Pipe::Start
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

fn traverse_pipes(pipes: &PipeGrid, start_i: usize, start_j: usize, mut dir_to: Dir) -> Vec<u64> {
    let mut dists = vec![]; // leave off initial zero so that reversing it aligns the two vectors
    let mut counter = 1;
    let (mut i, mut j) = update_i_j(start_i, start_j, dir_to);
    while (i, j) != (start_i, start_j) {
        dists.push(counter);
        counter += 1;
        let dir_from = dir_to.opposite();
        dir_to = pipes[i][j].where_to(dir_from);
        (i, j) = update_i_j(i, j, dir_to);
    }
    dists
}

fn compute_1(contents: &String) -> u64 {
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
            .filter(|d| **d != check_dir)
            .map(|d| Pipe::new_pipe(*d, check_dir))
            .any(|p| p == pipe_to_check)
        {
            dirs.push(check_dir);
        }
    }
    assert!(dirs.len() == 2);

    let dists_1 = traverse_pipes(&pipes, s_i, s_j, dirs[0]);
    let dists_2 = traverse_pipes(&pipes, s_i, s_j, dirs[1]);

    *dists_1
        .iter()
        .zip(dists_2.iter().rev())
        .map(|(d1, d2)| std::cmp::min(d1, d2))
        .max()
        .unwrap()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d10.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(6903, result);
    println!("part 1: {result}");

    // let result = compute_2(contents);
    // assert_eq!(973, result);
    // println!("part 2: {result}");
}
