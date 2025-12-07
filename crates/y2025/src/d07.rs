use std::collections::{HashMap, HashSet};

use shared::Problem;

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Splitter,
    Empty,
}

fn parse_input(contents: &str) -> (Vec<Vec<Element>>, usize) {
    let mut start_index = 0;
    let diagram = contents
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(i, c)| match c {
                    'S' => {
                        start_index = i;
                        Element::Empty
                    }
                    '^' => Element::Splitter,
                    '.' => Element::Empty,
                    _ => panic!("Unknown character {c}"),
                })
                .collect()
        })
        .collect();
    (diagram, start_index)
}

fn compute_1(contents: &str) -> u64 {
    let (diagram, start) = parse_input(contents);
    let mut tachyon_beams: HashSet<usize> = HashSet::new();
    tachyon_beams.insert(start);

    let mut split_count = 0;
    for diagram_row in diagram.into_iter() {
        let mut new_beams = HashSet::new();
        for tachyon_beam in tachyon_beams {
            match diagram_row[tachyon_beam] {
                Element::Splitter => {
                    split_count += 1;
                    // Assumption: splitters never occur on the edges
                    new_beams.insert(tachyon_beam - 1);
                    new_beams.insert(tachyon_beam + 1);
                }
                Element::Empty => {
                    new_beams.insert(tachyon_beam);
                }
            }
        }
        tachyon_beams = new_beams;
    }

    split_count
}

fn compute_2(contents: &str) -> u64 {
    let (diagram, start) = parse_input(contents);
    let mut tachyon_beams: HashMap<usize, u64> = HashMap::new();
    tachyon_beams.insert(start, 1);

    for diagram_row in diagram.into_iter() {
        let mut new_beams = HashMap::new();
        for (tachyon_beam, count) in tachyon_beams {
            match diagram_row[tachyon_beam] {
                Element::Splitter => {
                    // Assumption: splitters never occur on the edges
                    *new_beams.entry(tachyon_beam - 1).or_insert(0) += count;
                    *new_beams.entry(tachyon_beam + 1).or_insert(0) += count;
                }
                Element::Empty => {
                    *new_beams.entry(tachyon_beam).or_insert(0) += count;
                }
            }
        }
        tachyon_beams = new_beams;
    }
    tachyon_beams.values().sum::<u64>()
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
        "1667".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "62943905501815".to_string()
    }
}
