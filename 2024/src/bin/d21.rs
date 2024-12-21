use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    i: i64,
    j: i64,
}

impl Point {
    fn down(&self) -> Point {
        Point {
            i: self.i + 1,
            j: self.j,
        }
    }
    fn up(&self) -> Point {
        Point {
            i: self.i - 1,
            j: self.j,
        }
    }
    fn left(&self) -> Point {
        Point {
            i: self.i,
            j: self.j - 1,
        }
    }
    fn right(&self) -> Point {
        Point {
            i: self.i,
            j: self.j + 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum MoveButton {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl MoveButton {
    fn values() -> Vec<Self> {
        // This is likely a smell, but not sure how to do it better
        vec![
            MoveButton::Up,
            MoveButton::Down,
            MoveButton::Left,
            MoveButton::Right,
            MoveButton::A,
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NumpadButton {
    A,
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
}

trait Button: Sized {
    fn location(&self) -> Point;
    fn towards(&self, other: &Self) -> [Option<(Self, MoveButton)>; 2];
    fn from_point(point: Point) -> Self;
}

impl Button for MoveButton {
    fn location(&self) -> Point {
        match self {
            MoveButton::Up => Point { i: 0, j: 1 },
            MoveButton::Down => Point { i: 1, j: 1 },
            MoveButton::Left => Point { i: 1, j: 0 },
            MoveButton::Right => Point { i: 1, j: 2 },
            MoveButton::A => Point { i: 0, j: 2 },
        }
    }

    fn from_point(point: Point) -> Self {
        match point {
            Point { i: 0, j: 1 } => MoveButton::Up,
            Point { i: 1, j: 1 } => MoveButton::Down,
            Point { i: 1, j: 0 } => MoveButton::Left,
            Point { i: 1, j: 2 } => MoveButton::Right,
            Point { i: 0, j: 2 } => MoveButton::A,
            _ => panic!(),
        }
    }

    //            +---+---+
    //            | ^ | A |
    //        +---+---+---+
    //        | < | v | > |
    //        +---+---+---+
    fn towards(&self, other: &Self) -> [Option<(MoveButton, MoveButton)>; 2] {
        let mut out = [None, None];
        let here = self.location();
        let there = other.location();
        if here.i < there.i {
            out[0] = Some((MoveButton::from_point(here.down()), Self::Down));
        } else if here.i > there.i && self != &Self::Left {
            out[0] = Some((MoveButton::from_point(here.up()), Self::Up));
        }
        if here.j < there.j {
            out[1] = Some((MoveButton::from_point(here.right()), Self::Right));
        } else if here.j > there.j && self != &Self::Up {
            out[1] = Some((MoveButton::from_point(here.left()), Self::Left));
        }
        out
    }
}

impl Button for NumpadButton {
    fn location(&self) -> Point {
        match self {
            NumpadButton::A => Point { i: 3, j: 2 },
            NumpadButton::B0 => Point { i: 3, j: 1 },
            NumpadButton::B1 => Point { i: 2, j: 0 },
            NumpadButton::B2 => Point { i: 2, j: 1 },
            NumpadButton::B3 => Point { i: 2, j: 2 },
            NumpadButton::B4 => Point { i: 1, j: 0 },
            NumpadButton::B5 => Point { i: 1, j: 1 },
            NumpadButton::B6 => Point { i: 1, j: 2 },
            NumpadButton::B7 => Point { i: 0, j: 0 },
            NumpadButton::B8 => Point { i: 0, j: 1 },
            NumpadButton::B9 => Point { i: 0, j: 2 },
        }
    }

    fn from_point(point: Point) -> Self {
        match point {
            Point { i: 3, j: 2 } => NumpadButton::A,
            Point { i: 3, j: 1 } => NumpadButton::B0,
            Point { i: 2, j: 0 } => NumpadButton::B1,
            Point { i: 2, j: 1 } => NumpadButton::B2,
            Point { i: 2, j: 2 } => NumpadButton::B3,
            Point { i: 1, j: 0 } => NumpadButton::B4,
            Point { i: 1, j: 1 } => NumpadButton::B5,
            Point { i: 1, j: 2 } => NumpadButton::B6,
            Point { i: 0, j: 0 } => NumpadButton::B7,
            Point { i: 0, j: 1 } => NumpadButton::B8,
            Point { i: 0, j: 2 } => NumpadButton::B9,
            _ => panic!(),
        }
    }

    //        +---+---+---+
    //        | 7 | 8 | 9 |
    //        +---+---+---+
    //        | 4 | 5 | 6 |
    //        +---+---+---+
    //        | 1 | 2 | 3 |
    //        +---+---+---+
    //            | 0 | A |
    //            +---+---+
    fn towards(&self, other: &Self) -> [Option<(NumpadButton, MoveButton)>; 2] {
        let mut out = [None, None];
        let here = self.location();
        let there = other.location();
        if here.i < there.i && self != &Self::B1 {
            out[0] = Some((NumpadButton::from_point(here.down()), MoveButton::Down));
        } else if here.i > there.i {
            out[0] = Some((NumpadButton::from_point(here.up()), MoveButton::Up));
        }
        if here.j < there.j {
            out[1] = Some((NumpadButton::from_point(here.right()), MoveButton::Right));
        } else if here.j > there.j && self != &Self::B0 {
            out[1] = Some((NumpadButton::from_point(here.left()), MoveButton::Left));
        }
        out
    }
}

fn parse_input(contents: &str) -> Vec<Vec<NumpadButton>> {
    contents
        .trim()
        .split('\n')
        .map(|line| {
            // Starts at button A
            line.chars()
                .map(|c| match c {
                    'A' => NumpadButton::A,
                    '0' => NumpadButton::B0,
                    '1' => NumpadButton::B1,
                    '2' => NumpadButton::B2,
                    '3' => NumpadButton::B3,
                    '4' => NumpadButton::B4,
                    '5' => NumpadButton::B5,
                    '6' => NumpadButton::B6,
                    '7' => NumpadButton::B7,
                    '8' => NumpadButton::B8,
                    '9' => NumpadButton::B9,
                    _ => panic!("Unexpected character {c}"),
                })
                .collect()
        })
        .collect()
}

fn code_to_num(code: &[NumpadButton]) -> u64 {
    let mut num = 0;
    for button in code {
        match button {
            NumpadButton::A => {}
            NumpadButton::B0 => {
                num *= 10;
                num += 0;
            }
            NumpadButton::B1 => {
                num *= 10;
                num += 1;
            }
            NumpadButton::B2 => {
                num *= 10;
                num += 2;
            }
            NumpadButton::B3 => {
                num *= 10;
                num += 3;
            }
            NumpadButton::B4 => {
                num *= 10;
                num += 4;
            }
            NumpadButton::B5 => {
                num *= 10;
                num += 5;
            }
            NumpadButton::B6 => {
                num *= 10;
                num += 6;
            }
            NumpadButton::B7 => {
                num *= 10;
                num += 7;
            }
            NumpadButton::B8 => {
                num *= 10;
                num += 8;
            }
            NumpadButton::B9 => {
                num *= 10;
                num += 9;
            }
        }
    }
    num
}

type LevelCost = HashMap<(MoveButton, MoveButton), u64>;

#[derive(Debug, PartialEq, Eq)]
struct Step<T> {
    cost: u64,
    destination: T,
    move_step: MoveButton,
}

impl<T: PartialEq + Eq> PartialOrd for Step<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PartialEq + Eq> Ord for Step<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn get_level_0_costs() -> LevelCost {
    let mut level_0_costs: LevelCost = HashMap::new();
    for b0 in MoveButton::values() {
        for b1 in MoveButton::values() {
            level_0_costs.insert((b0, b1), 1);
        }
    }
    level_0_costs
}

fn next_level_costs(cost_map: LevelCost) -> LevelCost {
    let mut next_level_cost_map: LevelCost = HashMap::new();

    for curr in MoveButton::values() {
        'dest_loop: for dest in MoveButton::values() {
            if curr == dest {
                next_level_cost_map.insert((curr, dest), 1);
                continue;
            }
            let mut todo: BinaryHeap<Reverse<Step<MoveButton>>> = BinaryHeap::new();
            for (new_loc, move_step) in curr.towards(&dest).into_iter().flatten() {
                todo.push(Reverse(Step {
                    cost: cost_map[&(MoveButton::A, move_step)],
                    destination: new_loc,
                    move_step,
                }));
            }
            while let Some(Reverse(step)) = todo.pop() {
                if step.destination == dest && step.move_step != MoveButton::A {
                    todo.push(Reverse(Step {
                        cost: step.cost + cost_map[&(step.move_step, MoveButton::A)],
                        destination: step.destination,
                        move_step: MoveButton::A,
                    }));
                } else if step.destination == dest {
                    next_level_cost_map.insert((curr, dest), step.cost);
                    continue 'dest_loop;
                }
                for (new_loc, move_step) in step.destination.towards(&dest).into_iter().flatten() {
                    todo.push(Reverse(Step {
                        cost: step.cost + cost_map[&(step.move_step, move_step)],
                        destination: new_loc,
                        move_step,
                    }));
                }
            }
            unreachable!()
        }
    }

    next_level_cost_map
}

fn cost_of_code(code: &[NumpadButton], cost_map: &LevelCost) -> u64 {
    let mut total_cost = 0;

    let mut curr = &NumpadButton::A;
    for dest in code.iter() {
        if curr == dest {
            total_cost += 1;
            curr = dest;
        } else {
            let mut todo: BinaryHeap<Reverse<Step<NumpadButton>>> = BinaryHeap::new();
            for (new_loc, move_step) in curr.towards(dest).into_iter().flatten() {
                todo.push(Reverse(Step {
                    cost: cost_map[&(MoveButton::A, move_step)],
                    destination: new_loc,
                    move_step,
                }));
            }
            while let Some(Reverse(step)) = todo.pop() {
                if &step.destination == dest && step.move_step != MoveButton::A {
                    todo.push(Reverse(Step {
                        cost: step.cost + cost_map[&(step.move_step, MoveButton::A)],
                        destination: step.destination,
                        move_step: MoveButton::A,
                    }));
                } else if &step.destination == dest {
                    curr = dest;
                    total_cost += step.cost;
                    break;
                }
                for (new_loc, move_step) in step.destination.towards(dest).into_iter().flatten() {
                    todo.push(Reverse(Step {
                        cost: step.cost + cost_map[&(step.move_step, move_step)],
                        destination: new_loc,
                        move_step,
                    }));
                }
            }
        }
    }

    total_cost
}

fn solve(codes: Vec<Vec<NumpadButton>>, num_intermediates: u8) -> u64 {
    let mut out = 0;
    let mut level_costs = get_level_0_costs();
    for _ in 0..num_intermediates {
        level_costs = next_level_costs(level_costs);
    }
    for code in codes.iter() {
        let cost = cost_of_code(code, &level_costs);
        out += cost * code_to_num(code);
    }

    out
}

fn compute_1(contents: &str) -> u64 {
    solve(parse_input(contents), 2)
}

fn compute_2(contents: &str) -> u64 {
    solve(parse_input(contents), 25)
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d21.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(248684, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(307055584161760, result);
    println!("part 2: {result}");
}
