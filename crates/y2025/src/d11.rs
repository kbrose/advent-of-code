use std::collections::HashMap;

use shared::Problem;

fn get_node_index(
    node_names: &mut HashMap<String, usize>,
    name: &str,
    edges: &mut Vec<Vec<usize>>,
    next_index: &mut usize,
) -> usize {
    if let Some(n) = node_names.get(name) {
        *n
    } else {
        node_names.insert(name.to_string(), *next_index);
        *next_index += 1;
        edges.push(vec![]);
        *next_index - 1
    }
}

struct ParseInputResult {
    edges: Vec<Vec<usize>>,
    you: usize,
    out: usize,
    svr: usize,
    dac: usize,
    fft: usize,
}

fn parse_input(contents: &str) -> ParseInputResult {
    let mut node_names: HashMap<String, usize> = HashMap::new();
    let mut edges: Vec<Vec<usize>> = Vec::new();
    let mut next_index: usize = 0;
    let mut you: usize = 0;
    let mut out: usize = 0;
    let mut svr: usize = 0;
    let mut dac: usize = 0;
    let mut fft: usize = 0;
    for line in contents.trim().lines() {
        let (source, dests) = line.split_once(": ").unwrap();
        let source_idx = get_node_index(&mut node_names, source, &mut edges, &mut next_index);
        if source == "you" {
            you = source_idx;
        }
        if source == "svr" {
            svr = source_idx;
        }
        if source == "dac" {
            dac = source_idx;
        }
        if source == "fft" {
            fft = source_idx;
        }
        for dest in dests.split(' ') {
            let dest_idx = get_node_index(&mut node_names, dest, &mut edges, &mut next_index);
            if dest == "out" {
                out = dest_idx;
            }
            if dest == "svr" {
                svr = dest_idx;
            }
            if dest == "dac" {
                dac = dest_idx;
            }
            if dest == "fft" {
                fft = dest_idx;
            }
            edges[source_idx].push(dest_idx);
        }
    }
    ParseInputResult {
        edges,
        you,
        out,
        svr,
        dac,
        fft,
    }
}

fn paths_from_a_to_b(
    edges: &[Vec<usize>],
    start: usize,
    end: usize,
    cache: &mut HashMap<usize, u64>,
) -> u64 {
    if start == end {
        1
    } else {
        if let Some(paths) = cache.get(&start) {
            *paths
        } else {
            let paths = edges[start]
                .iter()
                .map(|next| paths_from_a_to_b(edges, *next, end, cache))
                .sum();
            cache.insert(start, paths);
            paths
        }
    }
}

fn compute_1(contents: &str) -> u64 {
    let ParseInputResult {
        edges,
        you,
        out,
        svr: _,
        dac: _,
        fft: _,
    } = parse_input(contents);
    paths_from_a_to_b(&edges, you, out, &mut HashMap::new())
}

fn compute_2(contents: &str) -> u64 {
    let ParseInputResult {
        edges,
        you: _,
        out,
        svr,
        dac,
        fft,
    } = parse_input(contents);

    let paths_from_svr_to_dac = paths_from_a_to_b(&edges, svr, dac, &mut HashMap::new());
    let paths_from_svr_to_fft = paths_from_a_to_b(&edges, svr, fft, &mut HashMap::new());
    let paths_from_dac_to_fft = paths_from_a_to_b(&edges, dac, fft, &mut HashMap::new());
    let paths_from_fft_to_dac = paths_from_a_to_b(&edges, fft, dac, &mut HashMap::new());
    let paths_from_dac_to_out = paths_from_a_to_b(&edges, dac, out, &mut HashMap::new());
    let paths_from_fft_to_out = paths_from_a_to_b(&edges, fft, out, &mut HashMap::new());

    (paths_from_svr_to_dac * paths_from_dac_to_fft * paths_from_fft_to_out)
        + (paths_from_svr_to_fft * paths_from_fft_to_dac * paths_from_dac_to_out)
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
        "674".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "438314708837664".to_string()
    }
}
