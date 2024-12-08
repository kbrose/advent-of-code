use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    SlopeLeft,
    SlopeUp,
    SlopeRight,
    SlopeDown,
    Forest,
}

type Id = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    dist: usize,
    dest: Id,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    id: Id,
    is_start: bool,
    is_end: bool,
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct Graph {
    nodes: HashMap<Id, Node>,
    start_id: Id,
    end_id: Id,
}

fn simplify_graph(graph: Graph) -> Graph {
    let mut nodes = graph.nodes;
    let ids: Vec<Id> = nodes.keys().map(|id| *id).collect();
    let mut removed_ids: Vec<&Id> = Vec::new();
    for id in ids.iter() {
        if (nodes[id].edges.len() == 2) && (*id != graph.start_id) && (*id != graph.end_id) {
            let edge1 = nodes[id].edges[0];
            let edge2 = nodes[id].edges[1];
            let mut neighbor1 = nodes[&nodes[id].edges[0].dest].clone();
            let mut neighbor2 = nodes[&nodes[id].edges[1].dest].clone();
            macro_rules! update_edges {
                ($n1:ident, $n2:ident, $e:ident) => {
                    if $n1.is_adjacent(&nodes[id]) {
                        $n1.edges = $n1
                            .edges
                            .iter()
                            .map(|edge| {
                                if edge.dest == *id {
                                    Edge {
                                        dest: $n2.id,
                                        dist: edge.dist + $e.dist,
                                    }
                                } else {
                                    edge.clone()
                                }
                            })
                            .collect();
                    }
                };
            }
            update_edges!(neighbor1, neighbor2, edge2);
            update_edges!(neighbor2, neighbor1, edge1);
            nodes.insert(neighbor1.id, neighbor1);
            nodes.insert(neighbor2.id, neighbor2);
            removed_ids.push(id);
        }
    }
    for id in removed_ids {
        nodes.remove(id);
    }

    Graph {
        nodes,
        start_id: graph.start_id,
        end_id: graph.end_id,
    }
}

fn parse_input(contents: &String, slopes_are_slippery: bool) -> Graph {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    for line in contents.trim().split('\n') {
        map.push(
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Forest,
                    '.' => Tile::Path,
                    '<' => {
                        if slopes_are_slippery {
                            Tile::SlopeLeft
                        } else {
                            Tile::Path
                        }
                    }
                    '^' => {
                        if slopes_are_slippery {
                            Tile::SlopeUp
                        } else {
                            Tile::Path
                        }
                    }
                    '>' => {
                        if slopes_are_slippery {
                            Tile::SlopeRight
                        } else {
                            Tile::Path
                        }
                    }
                    'v' => {
                        if slopes_are_slippery {
                            Tile::SlopeDown
                        } else {
                            Tile::Path
                        }
                    }
                    c => panic!("Unexpected character '{c}'"),
                })
                .collect(),
        )
    }
    let mut nodes: Vec<Node> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            let mut handle_slope = |new_i, new_j| {
                // assert!(map[new_i][new_j] != Tile::Forest);
                nodes.push(Node {
                    id: (i, j),
                    edges: vec![Edge {
                        dist: 1,
                        dest: (new_i, new_j),
                    }],
                    // No slope is the start or end
                    is_start: false,
                    is_end: false,
                });
            };
            match tile {
                Tile::Path => {
                    nodes.push(Node {
                        id: (i, j),
                        is_start: i == 0,
                        is_end: i == map.len() - 1,
                        edges: [
                            (i.checked_sub(1), Some(j)),
                            (i.checked_add(1), Some(j)),
                            (Some(i), j.checked_sub(1)),
                            (Some(i), j.checked_add(1)),
                        ]
                        .into_iter()
                        .map(|(new_i, new_j)| match (new_i, new_j) {
                            (Some(new_i), Some(new_j)) => {
                                if (new_i < map.len())
                                    && (new_j < row.len())
                                    && (map[new_i][new_j] != Tile::Forest)
                                {
                                    Some((new_i, new_j))
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                        .filter(|id| id.is_some())
                        .map(|id| Edge {
                            dist: 1,
                            dest: id.unwrap(),
                        })
                        .collect(),
                    });
                }
                Tile::SlopeLeft => {
                    handle_slope(i, j - 1);
                }
                Tile::SlopeUp => {
                    handle_slope(i - 1, j); // yes, minus
                }
                Tile::SlopeRight => {
                    handle_slope(i, j + 1);
                }
                Tile::SlopeDown => {
                    handle_slope(i + 1, j); // yes, plus
                }
                Tile::Forest => {}
            };
        }
    }

    assert_eq!(1, nodes.iter().filter(|n| n.is_start).count());
    assert_eq!(1, nodes.iter().filter(|n| n.is_end).count());
    let start_id = nodes.iter().filter(|n| n.is_start).next().unwrap().id;
    let end_id = nodes.iter().filter(|n| n.is_end).next().unwrap().id;

    Graph {
        nodes: HashMap::from_iter(nodes.iter().map(|n| (n.id, n.clone()))),
        start_id,
        end_id,
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Path {
    curr: Id,
    visited: HashSet<Id>,
    dist: usize,
}

fn find_longest_path(graph: Graph) -> usize {
    let mut possible_paths: Vec<Path> = vec![Path {
        curr: graph.start_id,
        visited: HashSet::new(),
        dist: 0,
    }];
    let mut longest_path_dist: usize = 0;
    while possible_paths.len() > 0 {
        let mut path = possible_paths.pop().unwrap();
        if path.curr == graph.end_id {
            longest_path_dist = std::cmp::max(longest_path_dist, path.dist);
        }
        path.visited.insert(path.curr);
        let mut edges: Vec<&Edge> = graph.nodes[&path.curr]
            .edges
            .iter()
            .filter(|edge| !path.visited.contains(&edge.dest))
            .collect();
        // Small optimization: We'll reuse `path` on the final edge rather than cloning it.
        // Saves about 8% runtime on part 2.
        // NOTE: edges.len() - 1 may underflow, but that's fine b/c in
        //       that case the iterator is empty anyway
        for edge in edges.iter().take(edges.len() - 1) {
            let mut new_path = path.clone();
            new_path.curr = edge.dest;
            new_path.dist += edge.dist;
            possible_paths.push(new_path);
        }
        edges.pop().and_then(|edge| {
            path.curr = edge.dest;
            path.dist += edge.dist;
            possible_paths.push(path);
            Some(())
        });
    }

    longest_path_dist
}

fn compute_1(contents: &String) -> usize {
    let graph = simplify_graph(parse_input(contents, true));
    find_longest_path(graph)
}

fn compute_2(contents: &String) -> usize {
    let graph = simplify_graph(parse_input(contents, false));
    find_longest_path(graph)
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d23.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(2178, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(6486, result);
    println!("part 2: {result}");
}
