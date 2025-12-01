use std::collections::{HashMap, HashSet};

use shared::Problem;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    i: usize,
    j: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn rotate_left(&self) -> Dir {
        match self {
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
        }
    }
    fn rotate_right(&self) -> Dir {
        match self {
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Neighbors {
    up: Option<Point>,
    right: Option<Point>,
    down: Option<Point>,
    left: Option<Point>,
}

impl Neighbors {
    fn neighbors(&self) -> Vec<Point> {
        let mut v = vec![];
        macro_rules! add_if_some {
            ($field:ident) => {
                if let Some(p) = self.$field {
                    v.push(p);
                }
            };
        }
        add_if_some!(up);
        add_if_some!(right);
        add_if_some!(down);
        add_if_some!(left);
        v
    }
}

fn map_to_graph(map: &[Vec<u8>]) -> HashMap<Point, Neighbors> {
    let mut graph = HashMap::new();
    for (i, map_row) in map.iter().enumerate() {
        for (j, value) in map_row.iter().enumerate() {
            let point = Point { i, j };

            macro_rules! get_possible_neighbor {
                ($dir:expr) => {
                    if let Some(other_val) = map.get($dir.0).and_then(|row| row.get($dir.1)) {
                        if other_val == value {
                            Some(Point {
                                i: $dir.0,
                                j: $dir.1,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };
            }

            let up = get_possible_neighbor!((i.wrapping_sub(1), j));
            let down = get_possible_neighbor!((i + 1, j));
            let left = get_possible_neighbor!((i, j.wrapping_sub(1)));
            let right = get_possible_neighbor!((i, j + 1));

            graph.insert(
                point,
                Neighbors {
                    up,
                    right,
                    down,
                    left,
                },
            );
        }
    }
    graph
}

fn parse_input(contents: &str) -> Vec<Vec<u8>> {
    contents
        .trim()
        .split('\n')
        .map(|line| line.bytes().collect())
        .collect()
}

fn compute_1(contents: &str) -> usize {
    let map = parse_input(contents);
    let graph2 = map_to_graph(&map);

    let mut to_visit_global: HashSet<Point> = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            to_visit_global.insert(Point { i, j });
        }
    }

    let mut counter = 0;

    while !to_visit_global.is_empty() {
        let mut to_visit: Vec<Point> = Vec::new();
        let initial_point = *to_visit_global.iter().next().unwrap();
        to_visit.push(initial_point);
        to_visit_global.remove(&initial_point);
        let mut area = 0;
        let mut perimeter = 0;
        while let Some(curr_point) = to_visit.pop() {
            let neighbors = &graph2[&curr_point].neighbors();
            // let neighbors = &graph[&curr_point];
            area += 1;
            perimeter += 4 - neighbors.len();
            for neighbor in neighbors {
                if to_visit_global.contains(neighbor) {
                    to_visit.push(*neighbor);
                    to_visit_global.remove(neighbor);
                }
            }
        }
        counter += area * perimeter;
    }

    counter
}

fn compute_2(contents: &str) -> usize {
    let map = parse_input(contents);
    let graph2 = map_to_graph(&map);

    let mut to_visit_global_area_counter: HashSet<Point> = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            to_visit_global_area_counter.insert(Point { i, j });
        }
    }

    let mut counter = 0;

    while !to_visit_global_area_counter.is_empty() {
        // Count the area
        let mut area = 0;
        let mut all_points_in_region = Vec::new();
        {
            let mut to_visit: Vec<Point> = Vec::new();
            let initial_point = *to_visit_global_area_counter.iter().next().unwrap();
            to_visit.push(initial_point);
            to_visit_global_area_counter.remove(&initial_point);
            while let Some(curr_point) = to_visit.pop() {
                all_points_in_region.push(curr_point);
                let possible_neighbors = &graph2[&curr_point];
                area += 1;
                for neighbor in possible_neighbors.neighbors() {
                    if to_visit_global_area_counter.contains(&neighbor) {
                        to_visit.push(neighbor);
                        to_visit_global_area_counter.remove(&neighbor);
                    }
                }
            }
        }

        // Count the sides
        // The upper_left_most_point is guaranteed to be a corner.
        // We're going to do some maze-solution walking, i.e. keep left hand on wall
        let mut visited: HashSet<(Point, Dir)> = HashSet::new();
        let mut sides = 0;

        // This will sort points in lexicographic order, i.e. up and left
        all_points_in_region.sort();
        for mut curr_point in all_points_in_region.into_iter() {
            if !visited.contains(&(curr_point, Dir::Up)) {
                // We only want to look at previously unseen coordinates
                // that do NOT have a neighbor directly upwards
                if graph2[&curr_point].up.is_some() {
                    continue;
                }
                let mut left_hand_on_wall_dir = Dir::Up;
                while !visited.contains(&(curr_point, left_hand_on_wall_dir)) {
                    visited.insert((curr_point, left_hand_on_wall_dir));
                    let neighbors = &graph2[&curr_point];
                    macro_rules! get_possible_movements {
                        ($dir1:ident, $dir2:ident) => {
                            (
                                neighbors.$dir1,
                                neighbors
                                    .$dir1
                                    .and_then(|neighbor_point| graph2[&neighbor_point].$dir2),
                            )
                        };
                    }
                    let (next, next_next) = {
                        match left_hand_on_wall_dir {
                            Dir::Up => get_possible_movements!(right, up),
                            Dir::Right => get_possible_movements!(down, right),
                            Dir::Down => get_possible_movements!(left, down),
                            Dir::Left => get_possible_movements!(up, left),
                        }
                    };
                    if let Some(next_point) = next_next {
                        // This is where we round a (concave) corner
                        //
                        // E.g. let the arrow in the diagram mark the current
                        // position and direction of your hand
                        //
                        //     |
                        //     |
                        // ––––+
                        //     ^
                        //
                        // Goes to
                        //
                        //     |
                        //     |
                        // ––––+<
                        //
                        sides += 1; // we completed a side
                        curr_point = next_point;
                        left_hand_on_wall_dir = left_hand_on_wall_dir.rotate_left();
                    } else if let Some(next_point) = next {
                        // Continuing along the same wall
                        //
                        // |
                        // |<
                        // |
                        curr_point = next_point;
                    } else {
                        // Hit a dead end (i.e. convex corner)
                        //
                        // +–+
                        // |<|
                        sides += 1; // we completed a side
                        left_hand_on_wall_dir = left_hand_on_wall_dir.rotate_right();
                    }
                }
            }
        }

        counter += area * sides;
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
        "1363682".to_string()
    }
    fn expected2(&self) -> String {
        "787680".to_string()
    }
}
