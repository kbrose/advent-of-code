use std::str::FromStr;

use shared::Problem;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rule {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct RuleParseError;

impl FromStr for Rule {
    type Err = RuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once('|').ok_or(RuleParseError)?;
        Ok(Rule {
            x: x.parse().map_err(|_| RuleParseError)?,
            y: y.parse().map_err(|_| RuleParseError)?,
        })
    }
}

#[derive(Debug, Clone)]
struct Manual {
    pages: Vec<u8>,
}

#[derive(Debug)]
struct ManualParseError;

impl FromStr for Manual {
    type Err = ManualParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages = s
            .split(',')
            .map(|num_str| num_str.parse().unwrap())
            .collect();
        Ok(Manual { pages })
    }
}

type Manuals = Vec<Manual>;
type Rules = Vec<Rule>;

impl Manual {
    fn is_ordered(&self, rules: &Rules) -> bool {
        for i in 1..self.pages.len() {
            for j in 0..i {
                if rules.contains(&Rule {
                    x: self.pages[i],
                    y: self.pages[j],
                }) {
                    return false;
                }
            }
        }
        true
    }
}

fn parse_input(contents: &str) -> (Rules, Manuals) {
    let (rule_str, manual_str) = contents.trim().split_once("\n\n").unwrap();
    let rules = rule_str
        .split('\n')
        .map(|rule_str| rule_str.parse().unwrap())
        .collect();
    let manuals = manual_str
        .split('\n')
        .map(|manual_str| manual_str.parse().unwrap())
        .collect();

    (rules, manuals)
}

fn compute_1(contents: &str) -> u64 {
    let (rules, manuals) = parse_input(contents);
    manuals
        .iter()
        .filter(|manual| manual.is_ordered(&rules))
        .map(|manual| manual.pages[manual.pages.len() / 2] as u64)
        .sum()
}

fn fix_manual(manual: Manual, rules: &Rules) -> Manual {
    let mut pages = manual.pages;
    pages.sort_by(|p1, p2| {
        if rules.contains(&Rule { x: *p1, y: *p2 }) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    Manual { pages }
}

fn compute_2(contents: &str) -> u64 {
    let (rules, manuals) = parse_input(contents);
    manuals
        .iter()
        .filter(|manual| !manual.is_ordered(&rules))
        .map(|manual| fix_manual(manual.clone(), &rules).pages[manual.pages.len() / 2] as u64)
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
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected1(&self) -> String {
        "4569".to_string()
    }
    fn expected2(&self) -> String {
        "6456".to_string()
    }
}
