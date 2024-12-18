use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    i: usize,
    j: usize,
}

impl Point {
    fn neighbors(&self) -> [Point; 4] {
        [
            Point {
                i: self.i.wrapping_sub(1),
                j: self.j,
            },
            Point {
                i: self.i + 1,
                j: self.j,
            },
            Point {
                i: self.i,
                j: self.j.wrapping_sub(1),
            },
            Point {
                i: self.i,
                j: self.j + 1,
            },
        ]
    }
}

const BOUNDS: usize = 70;
const START: Point = Point { i: 0, j: 0 };
const END: Point = Point {
    i: BOUNDS,
    j: BOUNDS,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Edge(Point, u64);

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

fn parse_input(contents: &str) -> Vec<Point> {
    contents
        .trim()
        .lines()
        .map(|line| {
            let (i, j) = line.split_once(',').unwrap();
            Point {
                i: i.parse().unwrap(),
                j: j.parse().unwrap(),
            }
        })
        .collect()
}

fn find_cost_of_shortest_path(falling_ram: Vec<Point>) -> Option<u64> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(Edge(START, 0)));
    visited.extend(falling_ram);
    let checked_adjustment = |new_point: Point| {
        // Only check one side of the bounds,
        // we used wrapping_sub to guarantee that 0 - 1 == MAX
        if new_point.i > BOUNDS || new_point.j > BOUNDS {
            None
        } else {
            Some(new_point)
        }
    };
    while let Some(Reverse(edge)) = to_visit.pop() {
        let point = edge.0;
        let cost = edge.1;
        if point == END {
            return Some(cost);
        }
        // visited.insert(point);
        for neighbor in point.neighbors().into_iter().filter_map(checked_adjustment) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                to_visit.push(Reverse(Edge(neighbor, cost + 1)));
            }
        }
    }
    None
}

fn compute_1(contents: &str) -> u64 {
    let falling_ram = parse_input(contents);
    find_cost_of_shortest_path(falling_ram.into_iter().take(1024).collect()).unwrap()
}

fn compute_2(contents: &str) -> Point {
    let falling_ram = parse_input(contents);
    for i in 1024..(falling_ram.len()) {
        let x = find_cost_of_shortest_path(falling_ram.clone().into_iter().take(i + 1).collect());
        if x.is_none() {
            return falling_ram[i];
        }
    }
    panic!();
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d18.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(278, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(Point { i: 43, j: 12 }, result);
    println!("part 2: {},{}", result.i, result.j);
}
