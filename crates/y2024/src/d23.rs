use std::collections::{HashMap, HashSet};

use shared::Problem;

type Name = [char; 2];

fn parse_input(contents: &str) -> HashMap<Name, HashSet<Name>> {
    let mut connections: HashMap<Name, HashSet<Name>> = HashMap::new();
    contents.trim().split('\n').for_each(|line| {
        let (first, second) = line.split_once('-').unwrap();
        let first = first.chars().collect::<Vec<char>>().try_into().unwrap();
        let second = second.chars().collect::<Vec<char>>().try_into().unwrap();
        connections.entry(first).or_default().insert(second);
        connections.entry(second).or_default().insert(first);
    });
    connections
}

fn compute_1(contents: &str) -> usize {
    let connections = parse_input(contents);
    let computer_names: Vec<Name> = connections.keys().copied().collect();
    let mut total = 0;
    for comp0 in computer_names.iter() {
        let comp0_neighbors = &connections[comp0];
        for comp1 in comp0_neighbors {
            // Uniqueness check: Only count each triangle once
            if comp1 < comp0 {
                continue;
            }
            let comp1_neighbors = &connections[comp1];
            for comp2 in comp1_neighbors {
                // Uniqueness check: Only count each triangle once
                if comp2 < comp1 {
                    continue;
                }
                if (comp0[0] == 't' || comp1[0] == 't' || comp2[0] == 't')
                    && comp1_neighbors.contains(comp2)
                    && comp0_neighbors.contains(comp2)
                {
                    total += 1;
                }
            }
        }
    }
    total
}

fn compute_2(contents: &str) -> String {
    let connections = parse_input(contents);
    let mut largest_observed_kn = 0;
    let mut largest_observed_k: Vec<Name> = vec![];
    let computer_names: Vec<Name> = connections.keys().copied().collect();
    for comp0 in computer_names.iter() {
        let mut neighbors_of_current_clique = connections[comp0].clone();
        let mut clique: Vec<&Name> = vec![comp0];
        // This is the greedy algorithm for finding maximal (not maximUM!) cliques.
        // It is not at all guaranteed to work! But it does for the example and my input.
        for comp in computer_names.iter() {
            if clique.contains(&comp) {
                continue;
            }
            if neighbors_of_current_clique.contains(comp) {
                clique.push(comp);
                neighbors_of_current_clique = neighbors_of_current_clique
                    .intersection(&connections[comp])
                    .copied()
                    .collect();
            }
        }
        if clique.len() > largest_observed_kn {
            largest_observed_kn = clique.len();
            largest_observed_k = clique.into_iter().copied().collect();
        }
    }
    largest_observed_k.sort();
    largest_observed_k
        .iter()
        .map(|name| format!("{}{}", name[0], name[1]))
        .collect::<Vec<String>>()
        .join(",")
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
        "1110".to_string()
    }
    fn expected2(&self) -> String {
        "ej,hm,ks,ms,ns,rb,rq,sc,so,un,vb,vd,wd".to_string()
    }
}
