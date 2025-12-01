use shared::Problem;

fn parse_input(contents: &str) -> Vec<&str> {
    contents.trim().split(',').collect()
}

fn hash(s: &str) -> u8 {
    let mut out = std::num::Wrapping(0);
    for b in s.as_bytes().iter() {
        out += *b;
        out *= 17;
    }
    out.0
}

fn compute_1(contents: &str) -> u64 {
    parse_input(contents)
        .into_iter()
        .map(hash)
        .map(|h| h as u64)
        .sum()
}

#[derive(Clone, PartialEq, Eq)]
struct Lens {
    focal: u64,
    label: String,
}

fn index_of_lens(v: &[Lens], label: String) -> Option<usize> {
    for (i, element) in v.iter().enumerate() {
        if element.label == label {
            return Some(i);
        }
    }
    None
}

fn compute_2(contents: &str) -> u64 {
    let mut box_map: Vec<Vec<Lens>> = (0..256).map(|_| vec![]).collect();
    for command in parse_input(contents) {
        if command.ends_with('-') {
            let label = command.split('-').next().unwrap();
            let box_index = hash(label) as usize;
            if let Some(i) = index_of_lens(&box_map[box_index], label.to_string()) {
                box_map[box_index].remove(i);
            }
        } else {
            let mut command_split = command.split('=');
            let label = command_split.next().unwrap();
            let focal = command_split.next().unwrap().parse().unwrap();
            let lens = Lens {
                label: label.to_string(),
                focal,
            };
            let box_index = hash(label) as usize;
            match index_of_lens(&box_map[box_index], label.to_string()) {
                Some(i) => {
                    box_map[box_index][i] = lens;
                }
                None => {
                    box_map[box_index].push(lens);
                }
            }
        }
    }
    let mut summand = 0;
    for (mut box_index, lenses) in box_map.iter().enumerate() {
        box_index += 1;
        for (mut lens_index, lens) in lenses.iter().enumerate() {
            lens_index += 1;
            summand += box_index as u64 * lens_index as u64 * lens.focal;
        }
    }
    summand
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
        "518107".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "303404".to_string()
    }
}
