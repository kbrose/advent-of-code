use std::fs;

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
        if exp >= 1 {
            2_u64.pow(exp - 1)
        } else {
            0
        }
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
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let pickings = split
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    Card {
        winners,
        pickings,
        copies: 1,
    }
}

fn parse_input(contents: &String) -> Vec<Card> {
    contents
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(parse_line)
        .collect()
}

fn compute_1(contents: &String) -> u64 {
    let cards = parse_input(contents);
    cards.iter().map(|c| c.points()).sum()
}

fn compute_2(contents: String) -> u64 {
    let mut cards = parse_input(&contents);
    for i in 0..cards.len() {
        for j in i + 1..i + 1 + cards[i].copies_below() {
            cards[j].copies += cards[i].copies;
        }
    }
    cards.iter().map(|c| c.copies).sum()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d04.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(26914, result);
    println!("part 1: {result}");

    let result = compute_2(contents);
    assert_eq!(13080971, result);
    println!("part 2: {result}");
}
