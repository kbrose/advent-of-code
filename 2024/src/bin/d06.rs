use std::fs;

#[derive(Clone, PartialEq, Eq, Copy)]
enum Dir {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Obstacle,
    Unvisited,
    Visited(Dir),
}

#[derive(Clone, PartialEq, Eq)]
struct Guard {
    pos: (usize, usize),
    dir: Dir,
}

type Map = Vec<Vec<Tile>>;

fn parse_input(contents: &str) -> (Map, Guard) {
    let mut map: Map = Vec::new();
    let mut guard = Guard {
        pos: (usize::MAX, usize::MAX),
        dir: Dir::Up,
    };
    for (i, line) in contents.trim().split('\n').enumerate() {
        map.push(
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Tile::Unvisited,
                    '^' => {
                        guard.pos = (i, j);
                        Tile::Visited(Dir::Up)
                    }
                    '#' => Tile::Obstacle,
                    _ => panic!("Unexpected character {c}"),
                })
                .collect(),
        )
    }
    assert!(guard.pos.0 < usize::MAX);
    (map, guard)
}

fn process_path(map: &mut Map, mut guard: Guard) -> usize {
    loop {
        let (new_i, new_j) = match guard.dir {
            Dir::Left => (guard.pos.0, guard.pos.1 - 1),
            Dir::Up => (guard.pos.0 - 1, guard.pos.1),
            Dir::Right => (guard.pos.0, guard.pos.1 + 1),
            Dir::Down => (guard.pos.0 + 1, guard.pos.1),
        };
        if new_i >= map.len() || new_j >= map[0].len() {
            break;
        }
        if let Tile::Visited(dir) = map[new_i][new_j] {
            if dir == guard.dir {
                return 0;
            }
        }
        if map[new_i][new_j] == Tile::Obstacle {
            guard.dir = match guard.dir {
                Dir::Left => Dir::Up,
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
            };
        } else {
            map[new_i][new_j] = Tile::Visited(guard.dir);
            guard.pos = (new_i, new_j);
        }
    }
    map.iter()
        .map(|row| {
            row.iter()
                .filter(|tile| matches!(tile, Tile::Visited(_)))
                .count()
        })
        .sum()
}

fn compute_1(contents: &str) -> usize {
    let (mut map, guard) = parse_input(contents);
    process_path(&mut map, guard)
}

fn compute_2(contents: &str) -> u64 {
    let (map, guard) = parse_input(contents);
    let (initial_i, initial_j) = guard.pos;
    let mut counter = 0;
    let mut initial_path_map = map.clone();
    process_path(&mut initial_path_map, guard.clone());
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if i == initial_i && j == initial_j {
                continue;
            }
            if let Tile::Visited(_) = initial_path_map[i][j] {
                let mut new_map = map.clone();
                new_map[i][j] = Tile::Obstacle;
                if process_path(&mut new_map, guard.clone()) == 0 {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d06.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(4656, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(1575, result);
    println!("part 2: {result}");
}
