use std::collections::{HashSet, VecDeque};

use shared::Problem;

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

fn checked_adjustment(new_point: Point) -> Option<Point> {
    // Only check one side of the bounds,
    // we used wrapping_sub to guarantee that 0 - 1 == MAX
    if new_point.i > BOUNDS || new_point.j > BOUNDS {
        None
    } else {
        Some(new_point)
    }
}

fn find_cost_of_shortest_path(falling_ram: Vec<Point>) -> Option<u64> {
    let mut visited: HashSet<Point> = HashSet::new();
    // Djikstra on an unweighted graph is just BFS. Don't need a MaxHeap
    let mut to_visit = VecDeque::new();
    to_visit.push_back(Edge(START, 0));
    visited.extend(falling_ram);
    while let Some(edge) = to_visit.pop_front() {
        let point = edge.0;
        let cost = edge.1;
        if visited.contains(&point) {
            continue;
        } else if point == END {
            return Some(cost);
        }
        visited.insert(point);
        for neighbor in point.neighbors().into_iter().filter_map(checked_adjustment) {
            if !visited.contains(&neighbor) {
                // push_back is what makes this BFS. push_front would be DFS.
                to_visit.push_back(Edge(neighbor, cost + 1));
            }
        }
    }
    None
}

fn compute_1(contents: &str) -> u64 {
    let falling_ram = parse_input(contents);
    find_cost_of_shortest_path(falling_ram.into_iter().take(1024).collect()).unwrap()
}

fn compute_2(contents: &str) -> Option<Point> {
    let falling_ram = parse_input(contents);

    // Fill out the connected components as they exist at the end.
    type ConnectedComponentId = u16; // u16 is enough b/c 70^2 < 2^16
    let mut connected_component = [[ConnectedComponentId::MAX; BOUNDS + 1]; BOUNDS + 1];
    let mut curr_component_id = 0;
    for ram_point in falling_ram.iter() {
        connected_component[ram_point.i][ram_point.j] = curr_component_id;
        curr_component_id += 1;
    }
    for i in 0..=BOUNDS {
        for j in 0..=BOUNDS {
            let query_point = Point { i, j };
            if connected_component[query_point.i][query_point.j] == ConnectedComponentId::MAX {
                // Haven't found connected component for this point yet.
                // Fill it out using DFS.
                let mut visited: HashSet<Point> =
                    HashSet::from_iter(falling_ram.clone().into_iter());
                let mut to_visit = vec![query_point];
                while let Some(point) = to_visit.pop() {
                    if visited.contains(&point) {
                        continue;
                    }
                    visited.insert(point);

                    connected_component[point.i][point.j] = curr_component_id;
                    for neighbor in point.neighbors().into_iter().filter_map(checked_adjustment) {
                        if !visited.contains(&neighbor) {
                            to_visit.push(neighbor);
                        }
                    }
                }
                curr_component_id += 1;
            }
        }
    }

    // Iterate backwards over the RAM, re-connecting components that should
    // now be connected when we remove that particular RAM block.
    for ram_index in (0..falling_ram.len()).rev() {
        let point = falling_ram[ram_index];
        let point_connected_component_id = connected_component[point.i][point.j];
        for neighbor in point.neighbors().into_iter().filter_map(checked_adjustment) {
            if !falling_ram[..ram_index].contains(&neighbor) {
                let neighbor_connected_component_id = connected_component[neighbor.i][neighbor.j];
                for connected_component_i in connected_component.iter_mut() {
                    for connected_component_ij in connected_component_i.iter_mut() {
                        if connected_component_ij == &neighbor_connected_component_id {
                            *connected_component_ij = point_connected_component_id;
                        }
                    }
                }
            }
        }
        if connected_component[START.i][START.j] == connected_component[END.i][END.j] {
            return Some(falling_ram[ram_index]);
        }
    }
    None
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
        let p = compute_2(contents).unwrap();
        format!("{},{}", p.i, p.j)
    }
    fn expected1(&self) -> String {
        "278".to_string()
    }
    fn expected2(&self) -> String {
        "43,12".to_string()
    }
}
