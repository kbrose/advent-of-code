use std::collections::{HashMap, HashSet};

use shared::Problem;

#[derive(Debug, PartialEq, Eq)]
enum Element {
    Start,
    Splitter,
    Empty,
}

fn parse_input(contents: &str) -> Vec<Vec<Element>> {
    contents
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Element::Start,
                    '^' => Element::Splitter,
                    '.' => Element::Empty,
                    _ => panic!("Unknown character {c}"),
                })
                .collect()
        })
        .collect()
}

fn compute_1(contents: &str) -> u64 {
    let diagram = parse_input(contents);
    let mut tachyon_beams: HashSet<usize> = HashSet::new();
    for (i, element) in diagram[0].iter().enumerate() {
        if element == &Element::Start {
            tachyon_beams.insert(i);
        }
    }

    let num_cols = diagram[0].len();

    let mut split_count = 0;
    for diagram_row in diagram[1..].into_iter() {
        let mut new_beams = HashSet::new();
        for tachyon_beam in tachyon_beams {
            match diagram_row[tachyon_beam] {
                Element::Start => panic!("Unexpected Start in non-first row of diagram"),
                Element::Splitter => {
                    split_count += 1;
                    if tachyon_beam > 0 {
                        new_beams.insert(tachyon_beam - 1);
                    }
                    if tachyon_beam < num_cols {
                        new_beams.insert(tachyon_beam + 1);
                    }
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
    let diagram = parse_input(contents);
    let mut tachyon_beams: HashMap<usize, u64> = HashMap::new();
    for (i, element) in diagram[0].iter().enumerate() {
        if element == &Element::Start {
            tachyon_beams.insert(i, 1);
        }
    }

    let num_cols = diagram[0].len();

    for diagram_row in diagram[1..].into_iter() {
        let mut new_beams = HashMap::new();
        for (tachyon_beam, count) in tachyon_beams {
            match diagram_row[tachyon_beam] {
                Element::Start => panic!("Unexpected Start in non-first row of diagram"),
                Element::Splitter => {
                    if tachyon_beam > 0 {
                        let x = new_beams.entry(tachyon_beam - 1).or_insert(0);
                        *x += count;
                    }
                    if tachyon_beam < num_cols {
                        let x = new_beams.entry(tachyon_beam + 1).or_insert(0);
                        *x += count;
                    }
                }
                Element::Empty => {
                    let x = new_beams.entry(tachyon_beam).or_insert(0);
                    *x += count;
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
