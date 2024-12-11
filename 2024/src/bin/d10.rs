use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Id = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    dest: Id,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    id: Id,
    height: u8,
    edges: Vec<Edge>,
}

type Graph = HashMap<Id, Node>;

fn parse_input(contents: &str) -> Graph {
    let map: Vec<Vec<u8>> = contents
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => panic!("Unexpected character {c}"),
                })
                .collect()
        })
        .collect();

    let mut nodes: Vec<Node> = Vec::new();
    let checked_adjustment = |(new_i, new_j)| {
        // Only check one side of the bounds,
        // we used wrapping_sub to guarantee that 0 - 1 == MAX
        if new_i >= map.len() || new_j >= map[0].len() {
            None
        } else {
            Some((new_i, new_j))
        }
    };
    for (i, row) in map.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            let places_to_check = [
                checked_adjustment((i.wrapping_sub(1), j)),
                checked_adjustment((i + 1, j)),
                checked_adjustment((i, j.wrapping_sub(1))),
                checked_adjustment((i, j + 1)),
            ];
            let edges: Vec<Edge> = places_to_check
                .iter()
                .filter(|x| x.is_some())
                .filter_map(|maybe_indexes| {
                    if let Some((new_i, new_j)) = maybe_indexes {
                        if map[*new_i][*new_j] == height + 1 {
                            Some(Edge {
                                dest: (*new_i, *new_j),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            nodes.push(Node {
                id: (i, j),
                height: *height,
                edges,
            });
        }
    }

    // for node in nodes.iter() {
    //     println!("{node:?}");
    // }

    HashMap::from_iter(nodes.iter().map(|n| (n.id, n.clone())))
}

fn compute_1(contents: &str) -> usize {
    let graph = parse_input(contents);

    let mut score_sum = 0;

    for node in graph.values() {
        if node.height == 0 {
            let mut visited_nines: HashSet<Id> = HashSet::new();
            let mut to_visit: Vec<Id> = node.edges.iter().map(|e| e.dest).collect();
            while !to_visit.is_empty() {
                let curr = graph.get(&to_visit.pop().unwrap()).unwrap();
                if curr.height == 9 {
                    visited_nines.insert(curr.id);
                } else {
                    to_visit.extend(curr.edges.iter().map(|e| e.dest));
                }
            }
            score_sum += visited_nines.len()
        }
    }

    score_sum
}

fn compute_2(contents: &str) -> usize {
    let graph = parse_input(contents);

    let mut rating_sum = 0;

    for node in graph.values() {
        if node.height == 0 {
            let mut to_visit: Vec<Id> = node.edges.iter().map(|e| e.dest).collect();
            while !to_visit.is_empty() {
                let curr = graph.get(&to_visit.pop().unwrap()).unwrap();
                if curr.height == 9 {
                    rating_sum += 1
                } else {
                    to_visit.extend(curr.edges.iter().map(|e| e.dest));
                }
            }
        }
    }

    rating_sum
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d10.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(733, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(1514, result);
    println!("part 2: {result}");
}
