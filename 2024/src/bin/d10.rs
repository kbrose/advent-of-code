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

impl Node {
    fn is_adjacent(&self, other: &Self) -> bool {
        self.edges
            .iter()
            .map(|edge| edge.dest)
            .any(|dest| dest == other.id)
    }
}

type Graph = HashMap<Id, Node>;

// fn simplify_graph(graph: Graph) -> Graph {
//     let mut nodes = graph.nodes;
//     let ids: Vec<Id> = nodes.keys().copied().collect();
//     let mut removed_ids: Vec<&Id> = Vec::new();
//     for id in ids.iter() {
//         if (nodes[id].edges.len() == 2) && (*id != graph.start_id) && (*id != graph.end_id) {
//             let edge1 = nodes[id].edges[0];
//             let edge2 = nodes[id].edges[1];
//             let mut neighbor1 = nodes[&nodes[id].edges[0].dest].clone();
//             let mut neighbor2 = nodes[&nodes[id].edges[1].dest].clone();
//             macro_rules! update_edges {
//                 ($n1:ident, $n2:ident, $e:ident) => {
//                     if $n1.is_adjacent(&nodes[id]) {
//                         $n1.edges = $n1
//                             .edges
//                             .iter()
//                             .map(|edge| {
//                                 if edge.dest == *id {
//                                     Edge {
//                                         dest: $n2.id,
//                                         dist: edge.dist + $e.dist,
//                                     }
//                                 } else {
//                                     edge.clone()
//                                 }
//                             })
//                             .collect();
//                     }
//                 };
//             }
//             update_edges!(neighbor1, neighbor2, edge2);
//             update_edges!(neighbor2, neighbor1, edge1);
//             nodes.insert(neighbor1.id, neighbor1);
//             nodes.insert(neighbor2.id, neighbor2);
//             removed_ids.push(id);
//         }
//     }
//     for id in removed_ids {
//         nodes.remove(id);
//     }

//     Graph {
//         nodes,
//         start_id: graph.start_id,
//         end_id: graph.end_id,
//     }
// }

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

// #[derive(Debug, PartialEq, Eq, Clone)]
// struct Path {
//     curr: Id,
//     visited: HashSet<Id>,
//     dist: usize,
// }

// fn find_longest_path(graph: Graph) -> usize {
//     let mut possible_paths: Vec<Path> = vec![Path {
//         curr: graph.start_id,
//         visited: HashSet::new(),
//         dist: 0,
//     }];
//     let mut longest_path_dist: usize = 0;
//     while let Some(mut path) = possible_paths.pop() {
//         if path.curr == graph.end_id {
//             longest_path_dist = std::cmp::max(longest_path_dist, path.dist);
//         }
//         path.visited.insert(path.curr);
//         let mut edges: Vec<&Edge> = graph.nodes[&path.curr]
//             .edges
//             .iter()
//             .filter(|edge| !path.visited.contains(&edge.dest))
//             .collect();
//         // Small optimization: We'll reuse `path` on the final edge rather than cloning it.
//         // Saves about 8% runtime on part 2.
//         // NOTE: edges.len() - 1 may underflow, but that's fine b/c in
//         //       that case the iterator is empty anyway
//         for edge in edges.iter().take(edges.len() - 1) {
//             let mut new_path = path.clone();
//             new_path.curr = edge.dest;
//             new_path.dist += edge.dist;
//             possible_paths.push(new_path);
//         }
//         if let Some(edge) = edges.pop() {
//             path.curr = edge.dest;
//             path.dist += edge.dist;
//             possible_paths.push(path);
//         }
//     }

//     longest_path_dist
// }

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
    // assert_eq!(733, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    // assert_eq!(6486, result);
    println!("part 2: {result}");
}
