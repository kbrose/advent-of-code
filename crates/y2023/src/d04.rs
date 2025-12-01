use shared::Problem;

struct Card {
    winners: Vec<u64>,
    pickings: Vec<u64>,
    copies: u64,
}

impl Card {
    fn points(&self) -> u64 {
        let exp = self
            .pickings
            .iter()
            .filter(|p| self.winners.contains(p))
            .count() as u32;
        if exp >= 1 { 2_u64.pow(exp - 1) } else { 0 }
    }

    fn copies_below(&self) -> usize {
        self.pickings
            .iter()
            .filter(|p| self.winners.contains(p))
            .count()
    }
}

fn parse_line(line: &str) -> Card {
    let line = line.split(": ").nth(1).unwrap();
    let mut split = line.split(" | ");
    let winners = split
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let pickings = split
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    Card {
        winners,
        pickings,
        copies: 1,
    }
}

fn parse_input(contents: &str) -> Vec<Card> {
    contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect()
}

fn compute_1(contents: &str) -> u64 {
    let cards = parse_input(contents);
    cards.iter().map(|c| c.points()).sum()
}

fn compute_2(contents: &str) -> u64 {
    let mut cards = parse_input(&contents);
    for i in 0..cards.len() {
        for j in i + 1..i + 1 + cards[i].copies_below() {
            cards[j].copies += cards[i].copies;
        }
    }
    cards.iter().map(|c| c.copies).sum()
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
        "26914".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "13080971".to_string()
    }
}
