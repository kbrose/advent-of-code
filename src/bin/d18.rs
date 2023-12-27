use std::{
    cmp::{max, min},
    fs,
    str::FromStr,
};

#[derive(PartialEq, Eq, Hash, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDirError;

impl FromStr for Dir {
    type Err = ParseDirError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Dir::U),
            "D" => Ok(Dir::D),
            "L" => Ok(Dir::L),
            "R" => Ok(Dir::R),
            _ => Err(ParseDirError),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

struct Instruction {
    dir: Dir,
    count: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let dir = split
            .next()
            .ok_or_else(|| ParseInstructionError)?
            .parse()
            .or(Err(ParseInstructionError))?;
        let count = split
            .next()
            .ok_or_else(|| ParseInstructionError)?
            .parse()
            .or(Err(ParseInstructionError))?;
        Ok(Instruction { dir, count })
    }
}

fn parse_input(contents: &String) -> Vec<Point> {
    let (mut x, mut y) = (0, 0);
    let mut vertices = vec![Point { x: 0, y: 0 }];
    for instruction_string in contents.trim().split('\n') {
        let instruction: Instruction = instruction_string
            .parse()
            .expect("Malformed instruction string '{instruction_string}'");
        match instruction.dir {
            Dir::U => {
                vertices.push(Point {
                    x,
                    y: y + instruction.count,
                });
                y += instruction.count;
            }
            Dir::D => {
                vertices.push(Point {
                    x,
                    y: y - instruction.count,
                });
                y -= instruction.count;
            }
            Dir::L => {
                vertices.push(Point {
                    x: x - instruction.count,
                    y,
                });
                x -= instruction.count;
            }
            Dir::R => {
                vertices.push(Point {
                    x: x + instruction.count,
                    y,
                });
                x += instruction.count;
            }
        }
    }

    // Make sure it begins and ends at (0, 0)
    assert_eq!(vertices.first().unwrap(), &Point { x: 0, y: 0 });
    assert_eq!(vertices.last().unwrap(), &Point { x: 0, y: 0 });

    vertices
}

fn is_interior(point: Point, vertices: &Vec<Point>) -> bool {
    let mut intersection_counter = 0;
    for (p1, p2) in vertices.iter().zip(vertices.iter().skip(1)) {
        if min(p1.x, p2.x) > point.x {
            continue;
        }
        if p1.x == p2.x {
            // the y's vary
            // First, test if the point lies on the line. That's a simple "yes".
            if p1.x == point.x && (min(p1.y, p2.y) <= point.y && point.y <= max(p1.y, p2.y)) {
                return true;
            }
            // Treat vertical line segments as bottom-open top-closed.
            // This will handle situations like ,---' (intersects once) vs. ,---, (doesn't intersect)
            if min(p1.y, p2.y) < point.y && point.y <= max(p1.y, p2.y) {
                intersection_counter += 1;
            }
        } else {
            // the x's vary
            // The only way we intersect is if we lie on top of the line.
            // This case automatically count as an interior point, short circuit.
            if p1.y == point.y && (min(p1.x, p2.x) <= point.x && point.x <= max(p1.x, p2.x)) {
                return true;
            }
        }
    }
    intersection_counter % 2 == 1
}

fn compute_1(contents: &String) -> u64 {
    let vertices = parse_input(contents);

    let left_bound = vertices.iter().map(|p| p.x).min().unwrap();
    let right_bound = vertices.iter().map(|p| p.x).max().unwrap();
    let lower_bound = vertices.iter().map(|p| p.y).min().unwrap();
    let upper_bound = vertices.iter().map(|p| p.y).max().unwrap();

    let mut counter = 0;
    for y in lower_bound..upper_bound + 1 {
        for x in left_bound..right_bound + 1 {
            if is_interior(Point { x, y }, &vertices) {
                counter += 1;
            }
        }
    }
    counter
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d18.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(53844, result);
    println!("part 1: {result}");

    // let result = compute_2(&contents).expect("Unable to find solution for part 2!");
    // assert_eq!(1027, result);
    // println!("part 2: {result}");
}
