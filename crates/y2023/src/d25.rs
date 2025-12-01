use shared::Problem;

use std::collections::HashMap;

type Node = usize;
type AdjMat = Vec<Vec<u64>>;

fn parse_input(contents: &str) -> AdjMat {
    let mut curr_id: Node = 0;
    let mut name_to_id: HashMap<String, Node> = HashMap::new();
    // First, find all nodes
    for line in contents.trim().split('\n') {
        let (node1_name, neighbor_names) = line.split_once(": ").unwrap();
        name_to_id.entry(node1_name.to_string()).or_insert_with(|| {
            // println!("{curr_id}: {node1_name}");
            curr_id += 1;
            curr_id - 1
        });
        for neighbor_name in neighbor_names.split(' ') {
            name_to_id
                .entry(neighbor_name.to_string())
                .or_insert_with(|| {
                    // println!("{curr_id}: {neighbor_name}");
                    curr_id += 1;
                    curr_id - 1
                });
        }
    }
    // Then, build the graph
    let mut adj_mat = vec![vec![0; name_to_id.len()]; name_to_id.len()];
    for line in contents.trim().split('\n') {
        let (node1_name, neighbor_names) = line.split_once(": ").unwrap();
        let node1 = *name_to_id.get(node1_name).unwrap();
        for neighbor_name in neighbor_names.split(' ') {
            let node2 = *name_to_id.get(neighbor_name).unwrap();
            adj_mat[node1][node2] = 1;
            adj_mat[node2][node1] = 1;
        }
    }
    adj_mat
}

fn min_cut(mut adj_mat: AdjMat) -> usize {
    let mut final_adj_mat = adj_mat.clone();
    let mut removed = vec![false; adj_mat.len()];
    let mut global_min_cut = u64::MAX;
    let mut num_nodes_merged = vec![1; adj_mat.len()];
    let mut final_num_nodes_merged = vec![1; adj_mat.len()];

    // We need these during Min-Cut-Phase
    let mut visited: Vec<bool> = vec![false; adj_mat.len()];
    let mut dist_to_visited_set: Vec<u64> = vec![0; adj_mat.len()];

    for _ in 1..adj_mat.len() {
        // Min-Cut-Phase
        let mut s = usize::MAX;
        let mut t = (0..removed.len()).find(|i| !removed[*i]).unwrap();
        let mut min_cut = 0;
        for i in 0..visited.len() {
            visited[i] = false;
            dist_to_visited_set[i] = adj_mat[i][t];
        }
        for _ in 1..adj_mat.len() {
            visited[t] = true;
            let new_t = (0..adj_mat.len())
                .max_by_key(|i| {
                    if (removed[*i]) || (visited[*i]) {
                        0
                    } else {
                        dist_to_visited_set[*i]
                    }
                })
                .unwrap();
            if (removed[new_t]) || (visited[new_t]) {
                break;
            }
            s = t;
            t = new_t;
            min_cut = dist_to_visited_set[t];
            dist_to_visited_set[t] = 0;
            for (dist_i, edge_t_i) in dist_to_visited_set.iter_mut().zip(adj_mat[t].iter()) {
                *dist_i += edge_t_i;
            }
        }

        removed[t] = true;
        for i in 0..adj_mat.len() {
            adj_mat[s][i] += adj_mat[t][i];
            adj_mat[i][s] = adj_mat[s][i];
        }
        for i in 0..adj_mat.len() {
            adj_mat[t][i] = 0;
            adj_mat[i][t] = 0;
        }
        if min_cut < global_min_cut {
            global_min_cut = min_cut;
            for i in 0..adj_mat.len() {
                for j in 0..adj_mat[i].len() {
                    final_adj_mat[i][j] = adj_mat[i][j];
                }
                final_num_nodes_merged[i] = num_nodes_merged[i];
            }
        }
        num_nodes_merged[s] += num_nodes_merged[t];
        num_nodes_merged[t] = 0;
    }

    let mut partition: Vec<usize> = Vec::new();
    let mut to_visit: Vec<usize> = Vec::new();
    to_visit.push(
        (0..final_adj_mat.len())
            .find(|i| final_adj_mat[*i].iter().sum::<u64>() > 0) // deleted nodes have no edges
            .unwrap(),
    );
    while !to_visit.is_empty() {
        let node = &to_visit.pop().unwrap();
        partition.push(*node);
        for (neighbor, neighbor_edges) in final_adj_mat.iter().enumerate() {
            if (neighbor_edges[*node] > 0)
                && (!to_visit.contains(&neighbor))
                && (!partition.contains(&neighbor))
            {
                to_visit.push(neighbor);
            }
        }
    }
    let x = partition
        .into_iter()
        .map(|i| final_num_nodes_merged[i])
        .sum::<usize>();
    let y = final_adj_mat.len() - x;

    x * y
}

fn compute_1(contents: &str) -> usize {
    let adj_mat = parse_input(contents);
    min_cut(adj_mat)
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
        "543834".to_string()
    }
    fn solve2(&self, _: &str) -> String {
        "no part 2".to_string()
    }
    fn expected2(&self) -> String {
        "no part 2".to_string()
    }
}
