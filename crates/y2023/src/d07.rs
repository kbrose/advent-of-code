use shared::Problem;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [i8; 5],
    bid: u64,
}

impl Hand {
    fn type_rank_1(&self) -> u8 {
        let mut counts: HashMap<i8, u8> = HashMap::new();
        for c in self.cards {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut counts: Vec<u8> = counts.values().copied().collect();
        counts.sort();
        counts.reverse();
        if counts[0] == 5 {
            7
        } else if counts[0] == 4 {
            6
        } else if counts[0] == 3 && counts[1] == 2 {
            5
        } else if counts[0] == 3 {
            4
        } else if counts[0] == 2 && counts[1] == 2 {
            3
        } else if counts[0] == 2 {
            2
        } else {
            1
        }
    }

    fn type_rank_2(&self) -> u8 {
        let mut counts: HashMap<i8, u8> = HashMap::new();
        let mut joker_count = 0;
        for c in self.cards {
            if c == -1 {
                // Joker
                joker_count += 1;
            } else {
                *counts.entry(c).or_insert(0) += 1;
            }
        }
        let mut counts: Vec<u8> = counts.values().copied().collect();
        counts.sort();
        counts.reverse();
        if counts.is_empty() {
            counts = vec![5];
        } else {
            counts[0] += joker_count; // Always best to add jokers to highest count
        }
        if counts[0] == 5 {
            7
        } else if counts[0] == 4 {
            6
        } else if counts[0] == 3 && counts[1] == 2 {
            5
        } else if counts[0] == 3 {
            4
        } else if counts[0] == 2 && counts[1] == 2 {
            3
        } else if counts[0] == 2 {
            2
        } else {
            1
        }
    }

    fn compare_1(&self, other: &Self) -> Ordering {
        self.type_rank_1()
            .cmp(&other.type_rank_1())
            .then(self.cards.cmp(&other.cards))
    }

    fn compare_2(&self, other: &Self) -> Ordering {
        self.type_rank_2()
            .cmp(&other.type_rank_2())
            .then(self.cards.cmp(&other.cards))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_1(other)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line_into_hand(line: &str) -> Hand {
    let mut splits = line.split_whitespace();
    let mut cards_iter = splits.next().unwrap().chars().map(|c| match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!(),
    });
    let c1 = cards_iter.next().unwrap();
    let c2 = cards_iter.next().unwrap();
    let c3 = cards_iter.next().unwrap();
    let c4 = cards_iter.next().unwrap();
    let c5 = cards_iter.next().unwrap();
    let cards = [c1, c2, c3, c4, c5];
    let bid = splits.next().unwrap().parse().unwrap();
    Hand { cards, bid }
}

fn parse_input(contents: &str) -> Vec<Hand> {
    contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(parse_line_into_hand)
        .collect()
}

fn compute_1(contents: &str) -> u64 {
    let mut hands = parse_input(contents);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank_minus_one, hand)| (rank_minus_one as u64 + 1) * hand.bid)
        .sum()
}

fn compute_2(contents: &str) -> u64 {
    let mut hands = parse_input(contents);
    hands = hands
        .into_iter()
        .map(|mut h| {
            for i in 0..h.cards.len() {
                if h.cards[i] == 9 {
                    h.cards[i] = -1; // Make jokers the lowest card
                }
            }
            h
        })
        .collect();
    hands.sort_by(|h1, h2| h1.compare_2(h2));
    hands
        .iter()
        .enumerate()
        .map(|(rank_minus_one, hand)| (rank_minus_one as u64 + 1) * hand.bid)
        .sum()
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
        "248105065".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "249515436".to_string()
    }
}
