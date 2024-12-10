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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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
            .ok_or(ParseInstructionError)?
            .parse()
            .or(Err(ParseInstructionError))?;
        let count = split
            .next()
            .ok_or(ParseInstructionError)?
            .parse()
            .or(Err(ParseInstructionError))?;
        Ok(Instruction { dir, count })
    }
}

struct Instruction2 {
    dir: Dir,
    count: i64,
}

impl FromStr for Instruction2 {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex_string = s
            .strip_suffix(")")
            .ok_or(ParseInstructionError)?
            .split("(#")
            .nth(1)
            .ok_or(ParseInstructionError)?;
        assert_eq!(hex_string.len(), 6);
        let count = i64::from_str_radix(&hex_string[0..5], 16).or(Err(ParseInstructionError))?;
        let dir = match hex_string.chars().nth(5).unwrap() {
            '0' => Dir::R,
            '1' => Dir::D,
            '2' => Dir::L,
            '3' => Dir::U,
            _ => return Err(ParseInstructionError),
        };
        Ok(Instruction2 { dir, count })
    }
}

fn parse_input(contents: &str) -> Vec<Point> {
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

fn parse_input_2(contents: &str) -> Vec<Point> {
    let (mut x, mut y) = (0, 0);
    let mut vertices = vec![Point { x: 0, y: 0 }];
    for instruction_string in contents.trim().split('\n') {
        let instruction: Instruction2 = instruction_string
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

fn compute_1(contents: &str) -> u64 {
    let vertices = parse_input(contents);

    let lower_bound = vertices.iter().map(|p| p.y).min().unwrap();
    let upper_bound = vertices.iter().map(|p| p.y).max().unwrap();

    let mut counter = 0;
    for y in (lower_bound..upper_bound + 1).rev() {
        counter += num_interior_points_on_horizontal(&vertices, y);
    }
    counter
}

fn num_interior_points_on_horizontal(vertices: &[Point], y: i64) -> u64 {
    // println!("");
    // All line segments that will intersect the horizontal ray at y
    let mut relevant_line_segments: Vec<(&Point, &Point)> = vertices
        .iter()
        .zip(vertices.iter().skip(1))
        .filter(|(p1, p2)| min(p1.y, p2.y) <= y && y <= max(p1.y, p2.y))
        .collect();

    // Order by x values
    relevant_line_segments.sort_by(|ls1, ls2| (ls1.0.x + ls1.1.x).cmp(&(ls2.0.x + ls2.1.x)));

    let mut counter = 0;
    let mut parity_change_points: Vec<i64> = vec![];

    for (i, (p1, p2)) in relevant_line_segments.iter().enumerate() {
        if p1.x == p2.x {
            // the y's vary
            // Treat vertical line segments as bottom-open top-closed.
            // This will handle situations like ,---' and '---, (parity should change)
            // vs. ,---, and '---' (parity does not change):
            //
            // line segments :  |  ,---'  '---,  ,---,  '---'
            // parity changes:  ^  ^          ^  ^   ^
            if min(p1.y, p2.y) < y && y <= max(p1.y, p2.y) {
                parity_change_points.push(p1.x);
            }
        } else {
            // the x's vary

            // This line must be in the interior of the relevant_line_segments array
            // So doing i-1 and i+1 is safe.

            let (prev_p1, prev_p2) = relevant_line_segments[i - 1];
            let (next_p1, next_p2) = relevant_line_segments[i + 1];

            let prev_came_from_up = prev_p1.y > y || prev_p2.y > y;
            let next_goes_to_up = next_p1.y > y || next_p2.y > y;
            let currently_exterior = parity_change_points.len() % 2 == 0;

            // The only way we intersect is if we lie on top of the line.
            // If we're already in the interior, then this line segment will
            // get counted anyway. Otherwise, we'll need to adjust our counting:
            //
            //  ,---'  (off) |  '---,  (off) |  ,---,  (off) |  '---'  (off)
            // iiooooo (n)   | oooooii (n)   | iioooii (n-1) | ooooooo (n+1)

            if currently_exterior {
                if (prev_came_from_up && !next_goes_to_up)
                    || (!prev_came_from_up && next_goes_to_up)
                {
                    counter += p1.x.abs_diff(p2.x);
                } else if !prev_came_from_up && !next_goes_to_up {
                    counter += p1.x.abs_diff(p2.x) - 1_u64;
                } else if prev_came_from_up && next_goes_to_up {
                    counter += p1.x.abs_diff(p2.x) + 1_u64;
                }
            }
        }
    }
    assert_eq!(parity_change_points.len() % 2, 0);
    for (x1, x2) in parity_change_points
        .iter()
        .step_by(2)
        .zip(parity_change_points.iter().skip(1).step_by(2))
    {
        counter += (*x2 - *x1) as u64 + 1;
    }
    counter
}

fn compute_2(contents: &str) -> u64 {
    let vertices = parse_input_2(contents);

    let lower_bound = vertices.iter().map(|p| p.y).min().unwrap();
    let upper_bound = vertices.iter().map(|p| p.y).max().unwrap();

    let mut counter = 0;
    for y in lower_bound..upper_bound + 1 {
        counter += num_interior_points_on_horizontal(&vertices, y);
    }
    counter
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d18.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(53844, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(42708339569950, result);
    println!("part 2: {result}");
}
