use std::fs;

fn parse_input(contents: &String) -> Vec<&str> {
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

fn compute_1(contents: &String) -> u64 {
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

fn index_of_lens(v: &Vec<Lens>, label: String) -> Option<usize> {
    for (i, element) in v.iter().enumerate() {
        if element.label == label {
            return Some(i);
        }
    }
    None
}

fn compute_2(contents: &String) -> u64 {
    let mut box_map: Vec<Vec<Lens>> = (0..256).map(|_| vec![]).collect();
    for command in parse_input(contents) {
        if command.ends_with('-') {
            let label = command.split('-').next().unwrap();
            let box_index = hash(label) as usize;
            match index_of_lens(&box_map[box_index], label.to_string()) {
                Some(i) => {
                    box_map[box_index].remove(i);
                }
                None => {}
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

fn main() {
    let contents =
        fs::read_to_string("inputs/d15.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(518107, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(303404, result);
    println!("part 2: {result}");
}
