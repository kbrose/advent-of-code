use std::{
    ops::{Add, Sub},
    str::FromStr,
};

use shared::Problem;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct PointParseError;

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // This works for both button and prize strings
        let (_, s) = s.split_once(": ").ok_or(PointParseError)?;
        let (x_str, y_str) = s.split_once(", ").ok_or(PointParseError)?;
        let x = x_str[2..].parse().map_err(|_| PointParseError)?;
        let y = y_str[2..].parse().map_err(|_| PointParseError)?;
        Ok(Point { x, y })
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn scalar_mul(&self, other: i64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

fn parse_input(contents: &str) -> Vec<Machine> {
    contents
        .trim()
        .split("\n\n")
        .map(|machine_str| {
            let (button_a_str, rest) = machine_str.split_once('\n').unwrap();
            let (button_b_str, target_str) = rest.split_once('\n').unwrap();
            Machine {
                a: button_a_str.parse().unwrap(),
                b: button_b_str.parse().unwrap(),
                prize: target_str.parse().unwrap(),
            }
        })
        .collect()
}

fn possible_score(machine: &Machine) -> Option<i64> {
    // let A be the vector of the A button.
    // Let B be the vector of the B button.
    // minimize(m * 3 + n) with integers m, n s.t.
    // 1. mA + nB = P
    // 2. m >= 0
    // 3. n >= 0
    //
    // However, even though this looks like an optimization problem,
    // there is actually exactly one solution (or <=1 solution when restricted to
    // non-negative integers) when A and B are not zero and not collinear:
    //
    // Let Ax be the x part of A, and similar for Ay, Bx, By, Px, Py.
    //
    // mAx = Px - nBx
    // mAy = Py - nBy
    //
    // m = (Px - nBx) / Ax
    // m = (Py - nBy) / Ay
    //
    // (Px - nBx) / Ax = (Py - nBy) / Ay
    // (Px - nBx)Ay = (Py - nBy)Ax  # multiply by AyAx
    // PxAy - nBxAy = PyAx - nByAx
    // PxAy - PyAx = n(BxAy - ByAx)
    // n = (PxAy - PyAx) / (BxAy - ByAx)
    //
    // Now that we know n, we can solve for m:
    //
    // m = (Py - nBy) / Ay
    //
    // The above math relies on Ax != 0, Ay != 0, and BxAy != ByAx
    // There's no reason this has to be true, but it does hold,
    // at least for my input.
    //
    // Note that these solutions may not be integers, so we can simply
    // check the final solution mA + nB = P, and if it is not satisfied
    // then it is unsolvable (given the constraints).
    let n = (machine.a.x * machine.prize.y - machine.prize.x * machine.a.y)
        / (machine.a.x * machine.b.y - machine.b.x * machine.a.y);
    let m = (machine.prize.x - n * machine.b.x) / machine.a.x;
    if (n >= 0) // might have found solution with negative n or m, ignore those
        && (m >= 0)
        && (&machine.a.scalar_mul(m) + &machine.b.scalar_mul(n) == machine.prize)
    {
        Some(3 * m + n)
    } else {
        None
    }
}

fn compute_1(contents: &str) -> i64 {
    let machines = parse_input(contents);
    machines.iter().filter_map(possible_score).sum()
}

fn compute_2(contents: &str) -> i64 {
    let machines = parse_input(contents);
    let shift = &Point {
        x: 10000000000000,
        y: 10000000000000,
    };
    let shift_machine = |mut machine: Machine| {
        machine.prize = &machine.prize + shift;
        machine
    };
    machines
        .into_iter()
        .filter_map(|m| possible_score(&shift_machine(m)))
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
        "26005".to_string()
    }
    fn expected2(&self) -> String {
        "105620095782547".to_string()
    }
}
