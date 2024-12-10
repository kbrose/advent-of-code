use std::{fs, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Calibration {
    target: i64,
    operands: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct CalibrationParseError;

impl FromStr for Calibration {
    type Err = CalibrationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target_str, operands_str) = s.split_once(": ").ok_or(CalibrationParseError)?;
        let target = target_str.parse().map_err(|_| CalibrationParseError)?;
        let operands = operands_str
            .split(' ')
            .rev() // NOTE! Reversing the iterator here, so we can .pop() off the end for the "first"
            .map(|s| s.parse::<i64>().map_err(|_| CalibrationParseError))
            .collect::<Result<Vec<i64>, CalibrationParseError>>()?;

        Ok(Calibration { target, operands })
    }
}

type Calibrations = Vec<Calibration>;

fn parse_input(contents: &str) -> Calibrations {
    contents
        .trim()
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect()
}

fn satisfiable(mut calibration: Calibration) -> bool {
    if calibration.operands.len() == 1 {
        // base case
        calibration.operands[0] == calibration.target
    } else {
        let operand = calibration.operands.pop().unwrap();
        let mut operands_plus = calibration.operands.clone();
        *operands_plus
            .get_mut(calibration.operands.len() - 1)
            .unwrap() += operand;
        if satisfiable(Calibration {
            target: calibration.target,
            operands: operands_plus,
        }) {
            true
        } else {
            let mut operands_mult = calibration.operands.clone();
            *operands_mult
                .get_mut(calibration.operands.len() - 1)
                .unwrap() *= operand;
            satisfiable(Calibration {
                target: calibration.target,
                operands: operands_mult,
            })
        }
    }
}

fn compute_1(contents: &str) -> i64 {
    let calibrations = parse_input(contents);
    calibrations
        .into_iter()
        .filter(|cal| satisfiable(cal.clone()))
        .map(|cal| cal.target)
        .sum()
}

// Who needs code re-use...
fn satisfiable_2(mut calibration: Calibration) -> bool {
    if calibration.operands.len() == 1 {
        // base case
        calibration.operands[0] == calibration.target
    } else {
        let operand = calibration.operands.pop().unwrap();
        let mut operands_plus = calibration.operands.clone();
        *operands_plus
            .get_mut(calibration.operands.len() - 1)
            .unwrap() += operand;
        if satisfiable_2(Calibration {
            target: calibration.target,
            operands: operands_plus,
        }) {
            true
        } else {
            let mut operands_mult = calibration.operands.clone();
            *operands_mult
                .get_mut(calibration.operands.len() - 1)
                .unwrap() *= operand;
            if satisfiable_2(Calibration {
                target: calibration.target,
                operands: operands_mult,
            }) {
                true
            } else {
                let mut operands_conc = calibration.operands.clone();
                let s1 = format!("{}", operand);
                let s2 = format!("{}", operands_conc[calibration.operands.len() - 1]);
                operands_conc[calibration.operands.len() - 1] =
                    format!("{s1}{s2}").parse().unwrap();
                satisfiable_2(Calibration {
                    target: calibration.target,
                    operands: operands_conc,
                })
            }
        }
    }
}

fn compute_2(contents: &str) -> i64 {
    let calibrations = parse_input(contents);
    calibrations
        .into_iter()
        .filter(|cal| satisfiable_2(cal.clone()))
        .map(|cal| cal.target)
        .sum()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d07.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(12940396350192, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(106016735664498, result);
    println!("part 2: {result}");
}
