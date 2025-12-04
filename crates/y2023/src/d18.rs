use shared::Problem;

use std::{
    cmp::{max, min},
    ops::{Add, Sub},
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

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
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
            Dir::U => y += instruction.count,
            Dir::D => y -= instruction.count,
            Dir::L => x -= instruction.count,
            Dir::R => x += instruction.count,
        }
        vertices.push(Point { x, y });
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

/// Possible ways to "corner", e.g. LeftUp means coming from Left, going Up
#[derive(Debug, PartialEq, Eq)]
enum DirectedTurn {
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn corner_type(p0: &Point, p1: &Point, p2: &Point) -> DirectedTurn {
    if p0.x < p1.x {
        if p2.y > p1.y {
            DirectedTurn::LeftUp
        } else {
            DirectedTurn::LeftDown
        }
    } else if p0.x > p1.x {
        if p2.y > p1.y {
            DirectedTurn::RightUp
        } else {
            DirectedTurn::RightDown
        }
    } else if p0.y > p1.y {
        if p2.x < p1.x {
            DirectedTurn::UpLeft
        } else {
            DirectedTurn::UpRight
        }
    } else {
        if p2.x < p1.x {
            DirectedTurn::DownLeft
        } else {
            DirectedTurn::DownRight
        }
    }
}

fn compute_2(contents: &str) -> u64 {
    let mut vertices = parse_input_2(contents);
    assert_eq!(vertices.first().unwrap(), &Point { x: 0, y: 0 });
    assert_eq!(vertices.last().unwrap(), &Point { x: 0, y: 0 });

    // "Correct" the placement of vertices. Assume that a vertex of (i, j)
    // in vertices means that the snow plow filled the area from (i, j) to (i+1, j+1)
    // on the coordinate grid. We first need to find the "handed-ness" of the polygon,
    // i.e. is it defined clockwise vs. counterclockwise. There's probably a better
    // way to do this, but I'm going to find the a lower left corner where I know
    // the exterior is to the left/down, and see which way the vertices are aligned.

    let mut low_left = *vertices.first().unwrap();
    let mut low_left_index = 0;
    for (i, p) in vertices.iter().enumerate() {
        if p.x < low_left.x || (p.x == low_left.x && p.y < low_left.y) {
            low_left = *p;
            low_left_index = i;
        }
    }
    let prev = if low_left_index == 0 {
        vertices[vertices.len() - 2]
    } else {
        vertices[low_left_index - 1]
    };
    let next = vertices[(low_left_index + 1) % vertices.len()];
    let is_clockwise = match corner_type(&prev, &low_left, &next) {
        DirectedTurn::RightUp => true,
        DirectedTurn::UpRight => false,
        _ => panic!("Inconceivable!"),
    };

    // Since we'll be iterating over (prev, curr, next) triples, tack item number 2 to the end.
    vertices.push(*vertices.iter().nth(1).unwrap());

    // This could be done in place with a little bit of extra book-keeping, but
    // I'm not going to worry about that...
    let mut new_vertices: Vec<Point> = Vec::with_capacity(vertices.len());
    for ((prev, curr), next) in vertices
        .iter()
        .zip(vertices.iter().skip(1))
        .zip(vertices.iter().skip(2))
    {
        // This here is the secret sauce. We're basically figuring out how to
        // translate the snow plow's (i, j) position, which actually means
        // the snow plow is clearing the square defined by (i, j) -> (i+1, j+1),
        // into an actual polygon's coordinate. The trick is that we need to adjust
        // the (i, j) coordinate by different amounts depending on the type
        // of corner we observed and the handed-ness of the polygon.
        //
        // E.g. a corner that looks like an L shape, with `prev` being right of the
        // corner and `next` being above the corner, requires no adjustment for a
        // clockwise-defined polygon, and an adjustment of +1, +1 for a
        // counter-clockwise-defined polygon:
        //
        //      clockwise     counter-clockwise
        //         ↑#               #↑
        //         ↑#               #↑
        //         ↑#               #+←←←
        //         ↑#####           #####
        //         +←←←←←
        let adjustment = match (corner_type(prev, curr, next), is_clockwise) {
            (DirectedTurn::LeftUp, true) => Point { x: 0, y: 1 },
            (DirectedTurn::LeftUp, false) => Point { x: 1, y: 0 },
            (DirectedTurn::LeftDown, true) => Point { x: 1, y: 1 },
            (DirectedTurn::LeftDown, false) => Point { x: 0, y: 0 },
            (DirectedTurn::RightUp, true) => Point { x: 0, y: 0 },
            (DirectedTurn::RightUp, false) => Point { x: 1, y: 1 },
            (DirectedTurn::RightDown, true) => Point { x: 1, y: 0 },
            (DirectedTurn::RightDown, false) => Point { x: 0, y: 1 },
            (DirectedTurn::UpLeft, true) => Point { x: 1, y: 0 },
            (DirectedTurn::UpLeft, false) => Point { x: 0, y: 1 },
            (DirectedTurn::UpRight, true) => Point { x: 1, y: 1 },
            (DirectedTurn::UpRight, false) => Point { x: 0, y: 0 },
            (DirectedTurn::DownLeft, true) => Point { x: 0, y: 0 },
            (DirectedTurn::DownLeft, false) => Point { x: 1, y: 1 },
            (DirectedTurn::DownRight, true) => Point { x: 0, y: 1 },
            (DirectedTurn::DownRight, false) => Point { x: 1, y: 0 },
        };
        new_vertices.push(*curr + adjustment);
    }
    new_vertices.push(*new_vertices.first().unwrap());

    // Finally, use a consequence of Stoke's theorem to compute the area.
    // I definitely did not know this off the top of my head, I found it on Wikipedia.
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let out = new_vertices
        .iter()
        .zip(new_vertices.iter().skip(1))
        .map(|(p0, p1)| p0.x * p1.y - p0.y * p1.x)
        .sum::<i64>()
        .unsigned_abs()
        / 2;

    out
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
        "53844".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "42708339569950".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_corner_type() {
        assert_eq!(
            corner_type(
                &Point { x: 0, y: 0 },
                &Point { x: 10, y: 0 },
                &Point { x: 10, y: -10 }
            ),
            DirectedTurn::LeftDown
        );
    }
}
