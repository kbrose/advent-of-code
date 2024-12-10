use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Add, Sub},
};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn scalar_mul(&self, other: i64) -> Pos {
        Pos {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Sub for &Pos {
    type Output = Pos;

    fn sub(self, other: &Pos) -> Self::Output {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for &Pos {
    type Output = Pos;

    fn add(self, other: &Pos) -> Self::Output {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn parse_input(contents: &str) -> (HashMap<char, Vec<Pos>>, usize, usize) {
    let mut antennas = HashMap::new();
    let mut i_lim = 0;
    let mut j_lim = 0;
    for (i, line) in contents.trim().split('\n').enumerate() {
        i_lim = i;
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push(Pos {
                    x: i as i64,
                    y: j as i64,
                });
            }
            j_lim = j;
        }
    }
    (antennas, i_lim + 1, j_lim + 1)
}

fn compute_1(contents: &str) -> usize {
    let (antennas, i_lim, j_lim) = parse_input(contents);

    let mut observed_spots: HashSet<Pos> = HashSet::new();
    for (_, antenna_positions) in antennas {
        for i in 1..antenna_positions.len() {
            for j in 0..i {
                let pos1 = &antenna_positions[i];
                let pos2 = &antenna_positions[j];
                let delta = pos2 - pos1;
                let candidates = [pos1 - &delta, pos2 + &delta];
                for candidate in candidates {
                    if candidate.x >= 0
                        && candidate.y >= 0
                        && (candidate.x as usize) < j_lim
                        && (candidate.y as usize) < i_lim
                    {
                        observed_spots.insert(candidate);
                    }
                }
            }
        }
    }
    observed_spots.len()
}

fn compute_2(contents: &str) -> usize {
    let (antennas, i_lim, j_lim) = parse_input(contents);

    let mut observed_spots: HashSet<Pos> = HashSet::new();
    for (_, antenna_positions) in antennas {
        for i in 1..antenna_positions.len() {
            for j in 0..i {
                let pos1 = &antenna_positions[i];
                let pos2 = &antenna_positions[j];
                let delta = pos2 - pos1;
                let mut multiplier = 0;
                loop {
                    let mut should_stop = true;
                    let candidates = [
                        pos1 - &(delta.scalar_mul(multiplier)),
                        pos2 + &(delta.scalar_mul(multiplier)),
                    ];
                    for candidate in candidates {
                        if candidate.x >= 0
                            && candidate.y >= 0
                            && (candidate.x as usize) < j_lim
                            && (candidate.y as usize) < i_lim
                        {
                            should_stop = false;
                            observed_spots.insert(candidate);
                        }
                    }
                    if should_stop {
                        break;
                    }
                    multiplier += 1;
                }
            }
        }
    }
    observed_spots.len()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d08.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(244, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(912, result);
    println!("part 2: {result}");
}
