use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashSet,
    fs,
    str::FromStr,
};

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
        let (_, s) = s.split_once("=").ok_or(PointParseError)?;
        let (x_str, y_str) = s.split_once(",").ok_or(PointParseError)?;
        let x = x_str.parse().map_err(|_| PointParseError)?;
        let y = y_str.parse().map_err(|_| PointParseError)?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Robot {
    pos: Point,
    vel: Point,
}

fn parse_input(contents: &str) -> Vec<Robot> {
    contents
        .trim()
        .split('\n')
        .map(|line| {
            let (pos_str, vel_str) = line.split_once(' ').unwrap();
            let pos = pos_str.parse().unwrap();
            let vel = vel_str.parse().unwrap();
            Robot { pos, vel }
        })
        .collect()
}

fn move_robot(robot: &mut Robot, num_steps: i64, width: i64, height: i64) {
    robot.pos = Point {
        x: (robot.pos.x + (robot.vel.x * num_steps)).rem_euclid(width),
        y: (robot.pos.y + (robot.vel.y * num_steps)).rem_euclid(height),
    }
}

fn quadrant_safety_score(robots: &[Robot], width: i64, height: i64) -> u64 {
    let mut up_left_count = 0;
    let mut up_right_count = 0;
    let mut low_left_count = 0;
    let mut low_right_count = 0;

    let width_split = width / 2;
    let height_split = height / 2;

    for robot in robots {
        match (
            (robot.pos.x.cmp(&width_split)),
            (robot.pos.y.cmp(&height_split)),
        ) {
            (Less, Less) => {
                up_left_count += 1;
            }
            (Less, Greater) => {
                low_left_count += 1;
            }
            (Greater, Less) => {
                up_right_count += 1;
            }
            (Greater, Greater) => {
                low_right_count += 1;
            }
            (Equal, _) => {}
            (_, Equal) => {}
        }
    }

    up_left_count * up_right_count * low_left_count * low_right_count
}

fn compute_1(contents: &str) -> u64 {
    let mut robots = parse_input(contents);
    println!();
    robots
        .iter_mut()
        .for_each(|robot| move_robot(robot, 100, WIDTH, HEIGHT));
    quadrant_safety_score(&robots, WIDTH, HEIGHT)
}

// Visual investigation to see what the tree actually looks like.
#[allow(dead_code)]
fn compute_2_visual(contents: &str) -> u64 {
    let mut robots = parse_input(contents);
    let mut counter = 0;
    let mut lowest_safety = u64::MAX;
    let step_size = 1;
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    loop {
        robots
            .iter_mut()
            .for_each(|robot| move_robot(robot, step_size, WIDTH, HEIGHT));
        counter += step_size;
        let curr_safety = quadrant_safety_score(&robots, WIDTH, HEIGHT);
        if curr_safety > lowest_safety {
            continue;
        }
        lowest_safety = curr_safety;
        println!("{counter}:");
        let positions: HashSet<Point> = robots.iter().map(|r| r.pos).collect();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if positions.contains(&Point { x, y }) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!();
        stdin.read_line(&mut buffer).unwrap();
    }
}

// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
// X                             X
// X                             X
// X                             X
// X                             X
// X              X              X
// X             XXX             X
// X            XXXXX            X
// X           XXXXXXX           X
// X          XXXXXXXXX          X
// X            XXXXX            X
// X           XXXXXXX           X
// X          XXXXXXXXX          X
// X         XXXXXXXXXXX         X
// X        XXXXXXXXXXXXX        X
// X          XXXXXXXXX          X
// X         XXXXXXXXXXX         X
// X        XXXXXXXXXXXXX        X
// X       XXXXXXXXXXXXXXX       X
// X      XXXXXXXXXXXXXXXXX      X
// X        XXXXXXXXXXXXX        X
// X       XXXXXXXXXXXXXXX       X
// X      XXXXXXXXXXXXXXXXX      X
// X     XXXXXXXXXXXXXXXXXXX     X
// X    XXXXXXXXXXXXXXXXXXXXX    X
// X             XXX             X
// X             XXX             X
// X             XXX             X
// X                             X
// X                             X
// X                             X
// X                             X
// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

fn has_32_robots_in_a_row(robots: &[Robot]) -> bool {
    let positions: HashSet<Point> = robots.iter().map(|r| r.pos).collect();
    positions.iter().any(|pos| {
        (1..32).all(|i| {
            positions.contains(&Point {
                x: pos.x,
                y: pos.y + i,
            })
        })
    })
}

fn compute_2(contents: &str) -> u64 {
    let mut robots = parse_input(contents);
    let mut step = 0;
    let mut lowest_safety = u64::MAX;
    let mut curr_safety = lowest_safety;
    while !has_32_robots_in_a_row(&robots) {
        // Safety check is not necessary, but takes runtime from 0.2s to 0.04s
        // It must be a lot faster to compute then has_32_robots_in_a_row()
        while curr_safety >= lowest_safety {
            step += 1;
            robots
                .iter_mut()
                .for_each(|robot| move_robot(robot, 1, WIDTH, HEIGHT));
            curr_safety = quadrant_safety_score(&robots, WIDTH, HEIGHT)
        }
        lowest_safety = curr_safety;
    }
    step
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d14.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(215987200, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(8050, result);
    println!("part 2: {result}");
}
