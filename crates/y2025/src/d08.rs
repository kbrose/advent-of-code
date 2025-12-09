use std::{collections::HashSet, str::FromStr};

use shared::Problem;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Node {
    x: u64,
    y: u64,
    z: u64,
}

impl Node {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    fn dist_squared(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x)).pow(2)
            + (self.y.abs_diff(other.y)).pow(2)
            + (self.z.abs_diff(other.z)).pow(2)
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let mut parse_next = || -> Result<u64, String> {
            Ok(parts
                .next()
                .ok_or("Not enough parts")?
                .parse::<u64>()
                .map_err(|_| "Unable to parse")?)
        };

        let x = parse_next()?;
        let y = parse_next()?;
        let z = parse_next()?;
        if let Some(_) = parts.next() {
            Err("Too many parts".to_string())
        } else {
            Ok(Self::new(x, y, z))
        }
    }
}

fn parse_input(contents: &str) -> Vec<Node> {
    contents
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn compute_1(contents: &str) -> u64 {
    let nodes = parse_input(contents);
    let mut edges = vec![vec![false; nodes.len()]; nodes.len()];
    let mut distances: Vec<(u64, usize, usize)> = Vec::with_capacity(nodes.len().pow(2));
    for (i, n0) in nodes.iter().enumerate() {
        for (j, n1) in nodes.iter().enumerate().skip(i) {
            if i != j {
                distances.push((n0.dist_squared(n1), i, j));
            }
        }
    }
    distances.sort_unstable();
    for i in 0..1000 {
        edges[distances[i].1][distances[i].2] = true;
        edges[distances[i].2][distances[i].1] = true;
    }
    let mut connected_component_sizes: Vec<u64> = Vec::new();
    let mut visited_overall: HashSet<usize> = HashSet::new();
    let mut to_visit_overall: Vec<usize> = Vec::from_iter(0..nodes.len());
    while !to_visit_overall.is_empty() {
        let curr = to_visit_overall.pop().unwrap();
        if !visited_overall.contains(&curr) {
            let mut curr_component_size = 0;
            let mut to_visit_component: Vec<usize> = vec![curr];
            let mut visited_component: HashSet<usize> = HashSet::new();
            while !to_visit_component.is_empty() {
                let curr = to_visit_component.pop().unwrap();
                if !visited_component.contains(&curr) {
                    curr_component_size += 1;
                    visited_component.insert(curr);
                    visited_overall.insert(curr);
                    to_visit_component.extend(
                        edges[curr]
                            .iter()
                            .enumerate()
                            .filter(|(_, edge)| **edge)
                            .map(|(i, _)| i),
                    );
                }
            }
            connected_component_sizes.push(curr_component_size);
        }
    }
    connected_component_sizes.sort();

    connected_component_sizes
        .into_iter()
        .rev()
        .take(3)
        .reduce(|a, b| a * b)
        .unwrap()
}

fn compute_2(contents: &str) -> u64 {
    let nodes = parse_input(contents);

    let mut distances: Vec<(u64, usize, usize)> = Vec::with_capacity(nodes.len().pow(2));
    for (i, n0) in nodes.iter().enumerate() {
        for (j, n1) in nodes.iter().enumerate().skip(i) {
            if i != j {
                distances.push((n0.dist_squared(n1), i, j));
            }
        }
    }
    distances.sort_unstable();
    distances.reverse();

    let mut connected_component_id = Vec::from_iter(0..nodes.len());

    let (final_node_i, final_node_j) = loop {
        let (_, i, j) = distances.pop().expect("Impossible!");
        let component_i = connected_component_id[i];
        let component_j = connected_component_id[j];
        if component_i != component_j {
            for k in 0..connected_component_id.len() {
                if connected_component_id[k] == component_i {
                    connected_component_id[k] = component_j;
                }
            }
        }
        if connected_component_id
            .iter()
            .skip(1)
            .all(|c| c == &connected_component_id[0])
        {
            break (nodes[i], nodes[j]);
        }
    };

    final_node_i.x * final_node_j.x
}

pub(crate) struct Day {}

impl Problem for Day {
    fn source_code_file(&self) -> String {
        file!().to_string()
    }
    fn solve1(&self, contents: &str) -> String {
        format!("{}", compute_1(contents))
    }
    fn expected1(&self) -> String {
        "79560".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "31182420".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_dist_squared() {
        assert_eq!(Node::new(0, 0, 0).dist_squared(&Node::new(0, 0, 0)), 0);
        assert_eq!(Node::new(0, 0, 0).dist_squared(&Node::new(1, 1, 0)), 2);
        assert_eq!(Node::new(0, 0, 0).dist_squared(&Node::new(0, 0, 2)), 4);
        assert_eq!(Node::new(1, 2, 3).dist_squared(&Node::new(3, 2, 1)), 8);
    }

    #[test]
    fn test_node_fromstr() {
        assert_eq!(Node::from_str("1,2,3").unwrap(), Node::new(1, 2, 3));
        assert_eq!(
            Node::from_str("100,200,300").unwrap(),
            Node::new(100, 200, 300)
        );
        assert!(Node::from_str("1,2").is_err());
        assert!(Node::from_str("1,2,3,4").is_err());
    }
}
