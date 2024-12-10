use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    Start,
    Stop,
    Neither,
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    left: usize,
    right: usize,
}

struct Input {
    start_index: usize,
    end_index: usize,
    moves: Vec<bool>,
    nodes: Vec<Node>,
}

struct Nodes {
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq, Eq)]
struct Cycle {
    warmup: usize,
    length: usize,
    terminals: Vec<usize>,
}

impl Nodes {
    fn initial_indexes(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.node_type == NodeType::Start)
            .map(|(i, _)| i)
            .collect()
    }
}

fn parse_inputs(contents: &str) -> Input {
    let mut lines = contents.split('\n').filter(|s| !s.is_empty());
    let moves = lines.next().unwrap().chars().map(|c| c == 'L').collect();
    let node_strings: Vec<&str> = lines.collect();
    // First pass, get the index of each name string
    let mut name_to_index: HashMap<&str, usize> = HashMap::new();
    let mut start_index = 0;
    let mut end_index = 0;
    for (i, node_string) in node_strings.iter().enumerate() {
        let node_name = &node_string[..3];
        if node_name == "AAA" {
            start_index = i;
        } else if node_name == "ZZZ" {
            end_index = i;
        }
        name_to_index.insert(node_name, i);
    }
    // Second pass, fill out node adjacencies
    let nodes = node_strings
        .into_iter()
        .map(|node_string| {
            let node_name = &node_string[..3];
            let left = name_to_index[&node_string[7..10]];
            let right = name_to_index[&node_string[12..15]];
            Node {
                left,
                right,
                node_type: {
                    if node_name.ends_with('A') {
                        NodeType::Start
                    } else if node_name.ends_with('Z') {
                        NodeType::Stop
                    } else {
                        NodeType::Neither
                    }
                },
            }
        })
        .collect();
    Input {
        start_index,
        end_index,
        moves,
        nodes,
    }
}

fn compute_1(contents: &str) -> u64 {
    let input = parse_inputs(contents);
    let mut start_index = input.start_index;
    let end_index = input.end_index;
    let mut moves = input.moves.iter().cycle();
    let nodes = input.nodes;
    let mut counter = 0;
    while start_index != end_index {
        counter += 1;
        if *moves.next().unwrap() {
            start_index = nodes[start_index].left;
        } else {
            start_index = nodes[start_index].right;
        }
    }
    counter
}

fn argmin(v: &[usize]) -> usize {
    let mut i = 0;
    let mut curr_min = usize::MAX;
    for (j, v_j) in v.iter().enumerate() {
        if *v_j < curr_min {
            i = j;
            curr_min = *v_j;
        }
    }
    i
}

fn compute_2(contents: String) -> usize {
    let input = parse_inputs(&contents);
    let nodes = Nodes { nodes: input.nodes };
    // nodes.curr_indexes = nodes.initial_indexes();

    let mut cycles = vec![];
    for mut node_index in nodes.initial_indexes() {
        let moves = input.moves.iter().enumerate().cycle();
        let mut visited: Vec<(usize, usize)> = vec![];
        for (i, move_left) in moves {
            let entry = (i, node_index);
            let pos = visited.iter().position(|el| el == &entry);
            if let Some(first_visited_index) = pos {
                cycles.push(Cycle {
                    warmup: first_visited_index,
                    length: visited.len() - first_visited_index,
                    terminals: visited[first_visited_index..]
                        .iter()
                        .map(|(_, n)| *n)
                        .enumerate()
                        .filter(|(_, n)| nodes.nodes[*n].node_type == NodeType::Stop)
                        .map(|(a, _)| a)
                        .collect(),
                });
                break;
            }
            visited.push(entry);
            if *move_left {
                node_index = nodes.nodes[node_index].left;
            } else {
                node_index = nodes.nodes[node_index].right;
            }
        }
    }

    // for a given cycle with warmup W, length L, and terminal T,
    // the valid ending steps are given by the formula
    // Wc + Tc + (Lc * i) for i = 0, 1, ...
    let lengths: Vec<usize> = cycles
        .iter()
        .flat_map(|c| c.terminals.iter().map(|_| c.length))
        .collect();
    let mut steps: Vec<usize> = cycles
        .iter()
        .flat_map(|c| c.terminals.iter().map(|t| c.warmup + t))
        .collect();
    while !steps.iter().all(|s| s == &steps[0]) {
        let cycle_index = argmin(&steps);
        steps[cycle_index] += lengths[cycle_index];
    }
    steps[0]
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d08.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(12599, result);
    println!("part 1: {result}");

    // People solved this using the LCM(...), but I'm pretty sure that relies
    // on assumptions not placed in the puzzle description... boo!
    let result = compute_2(contents);
    assert_eq!(8245452805243, result);
    println!("part 2: {result}");
}
