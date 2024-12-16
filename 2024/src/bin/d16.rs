use std::cmp::Reverse;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn rotate_left(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }
    fn rotate_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Id(usize, usize, Dir);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge(Id, u64);

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

fn parse_input(contents: &str) -> (Id, Id, HashMap<Id, Vec<Edge>>) {
    #[derive(PartialEq, Eq)]
    enum Tile {
        Start,
        End,
        Wall,
        Open,
    }
    let map: Vec<Vec<Tile>> = contents
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Open,
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    _ => panic!("Unexpected character {c}"),
                })
                .collect()
        })
        .collect();
    let mut graph = HashMap::new();
    let checked_adjustment = |(new_i, new_j)| {
        // Only check one side of the bounds,
        // we used wrapping_sub to guarantee that 0 - 1 == MAX
        if new_i >= map.len() || new_j >= map[0].len() {
            None
        } else {
            Some((new_i, new_j))
        }
    };
    let mut start = Id(usize::MAX, usize::MAX, Dir::Right);
    let mut end = Id(usize::MAX, usize::MAX, Dir::Up); // dir doesn't matter for end
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            match tile {
                Tile::End => {
                    end = Id(i, j, Dir::Up);
                    graph.insert(end, vec![]);
                }
                Tile::Wall => {}
                Tile::Open | Tile::Start => {
                    if tile == &Tile::Start {
                        start = Id(i, j, Dir::Right);
                    }
                    let places_to_check = [
                        checked_adjustment((i.wrapping_sub(1), j)).map(|inds| (inds, Dir::Up)),
                        checked_adjustment((i + 1, j)).map(|inds| (inds, Dir::Down)),
                        checked_adjustment((i, j.wrapping_sub(1))).map(|inds| (inds, Dir::Left)),
                        checked_adjustment((i, j + 1)).map(|inds| (inds, Dir::Right)),
                    ];
                    places_to_check.into_iter().for_each(|maybe_indexes| {
                        if let Some(((new_i, new_j), dir)) = maybe_indexes {
                            if map[new_i][new_j] != Tile::Wall {
                                graph
                                    .entry(Id(i, j, dir))
                                    .or_insert(vec![])
                                    .push(Edge(Id(new_i, new_j, dir), 1));
                                graph
                                    .entry(Id(i, j, dir.rotate_left()))
                                    .or_insert(vec![])
                                    .push(Edge(Id(i, j, dir), 1000));
                                graph
                                    .entry(Id(i, j, dir.rotate_right()))
                                    .or_insert(vec![])
                                    .push(Edge(Id(i, j, dir), 1000));
                            }
                        }
                    });
                }
            }
        }
    }
    (start, end, graph)
}

fn compute_1(contents: &str) -> u64 {
    let (start, end, graph) = parse_input(contents);
    let mut visited: HashSet<Id> = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(Edge(start, 0)));
    while let Some(Reverse(curr_edge)) = to_visit.pop() {
        let id = curr_edge.0;
        let cost = curr_edge.1;
        if id.0 == end.0 && id.1 == end.1 {
            return cost;
        }
        visited.insert(id);
        if graph.contains_key(&id) {
            for next_edge in graph[&id].iter() {
                let next_id = next_edge.0;
                let next_cost = next_edge.1;
                if !visited.contains(&next_id) {
                    to_visit.push(Reverse(Edge(next_id, cost + next_cost)));
                }
            }
        }
    }

    unreachable!()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TrackedEdge(Edge, HashSet<(usize, usize)>);

impl PartialOrd for TrackedEdge {
    fn partial_cmp(&self, other: &TrackedEdge) -> Option<Ordering> {
        Some(self.0 .1.cmp(&other.0 .1))
    }
}

impl Ord for TrackedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0 .1.cmp(&other.0 .1)
    }
}

fn compute_2(contents: &str) -> usize {
    let (start, end, graph) = parse_input(contents);
    let mut visited: HashSet<Id> = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(TrackedEdge(
        Edge(start, 0),
        HashSet::from([(start.0, start.1)]),
    )));
    let mut lowest_score = u64::MAX;
    let mut tiles_along_path: HashSet<(usize, usize)> = HashSet::new();
    while let Some(Reverse(curr_edge)) = to_visit.pop() {
        let id = curr_edge.0 .0;
        let cost = curr_edge.0 .1;
        let locations_visited = curr_edge.1;
        if cost > lowest_score {
            break;
        }
        if id.0 == end.0 && id.1 == end.1 {
            tiles_along_path.extend(locations_visited);
            lowest_score = cost;
            continue;
        }
        visited.insert(id);
        if graph.contains_key(&id) {
            for next_edge in graph[&id].iter() {
                let next_id = next_edge.0;
                let next_cost = next_edge.1;
                let mut next_locations_visited = locations_visited.clone();
                next_locations_visited.insert((next_id.0, next_id.1));
                if !visited.contains(&next_id) {
                    to_visit.push(Reverse(TrackedEdge(
                        Edge(next_id, cost + next_cost),
                        next_locations_visited,
                    )));
                }
            }
        }
    }

    tiles_along_path.len()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d16.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(102504, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(535, result);
    println!("part 2: {result}");
}
