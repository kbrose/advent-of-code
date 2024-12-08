use std::{
    fs,
    ops::{Add, Sub},
};

const TEST_AREA: [f64; 2] = [200000000000000.0, 400000000000000.0];

// #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i128,
    y: i128,
    z: i128,
}

impl Coord {
    fn cross_2d(&self, other: &Self) -> i128 {
        // self.x * other.y - self.y * other.x
        self.x
            .checked_mul(other.y)
            .unwrap()
            .checked_sub(self.y.checked_mul(other.x).unwrap())
            .unwrap()
    }

    fn cross(&self, other: &Self) -> Coord {
        Coord {
            x: self
                .y
                .checked_mul(other.z)
                .unwrap()
                .checked_sub(self.z.checked_mul(other.y).unwrap())
                .unwrap(),
            y: self
                .z
                .checked_mul(other.x)
                .unwrap()
                .checked_sub(self.x.checked_mul(other.z).unwrap())
                .unwrap(),
            z: self
                .x
                .checked_mul(other.y)
                .unwrap()
                .checked_sub(self.y.checked_mul(other.x).unwrap())
                .unwrap(),
        }
    }

    fn dot(&self, other: &Self) -> i128 {
        let x = self.x.checked_mul(other.x).unwrap();
        let y = self.y.checked_mul(other.y).unwrap();
        let z = self.z.checked_mul(other.z).unwrap();
        x.checked_add(y).unwrap().checked_add(z).unwrap()
    }

    fn scalar_mul(&self, other: i128) -> Coord {
        Coord {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }

    fn scalar_div(&self, other: i128) -> Coord {
        Coord {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Sub for &Coord {
    type Output = Coord;

    fn sub(self, other: Self) -> Self::Output {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for &Coord {
    type Output = Coord;

    fn add(self, other: Self) -> Self::Output {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCoordError;

impl std::str::FromStr for Coord {
    type Err = ParseCoordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y_z) = s.split_once(", ").ok_or(ParseCoordError)?;
        let (y, z) = y_z.split_once(", ").ok_or(ParseCoordError)?;

        let x_fromstr = x.trim().parse::<i128>().map_err(|_| ParseCoordError)?;
        let y_fromstr = y.trim().parse::<i128>().map_err(|_| ParseCoordError)?;
        let z_fromstr = z.trim().parse::<i128>().map_err(|_| ParseCoordError)?;

        Ok(Coord {
            x: x_fromstr,
            y: y_fromstr,
            z: z_fromstr,
        })
    }
}

// #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Hail {
    pos: Coord,
    vel: Coord,
}

impl Sub for &Hail {
    type Output = Hail;

    fn sub(self, other: Self) -> Self::Output {
        Hail {
            pos: &self.pos - &other.pos,
            vel: &self.vel - &other.vel,
        }
    }
}

fn intersection_point_2d(h1: &Hail, h2: &Hail) -> Option<(f64, f64)> {
    // I'm using the method described here:
    // https://stackoverflow.com/questions/563198
    let r_x_s = h1.vel.cross_2d(&h2.vel);
    let q_minus_p_x_r = (&h2.pos - &h1.pos).cross_2d(&h1.vel);
    if r_x_s == 0 {
        if q_minus_p_x_r == 0 {
            if (h1.vel.x > 0) == (h2.vel.x > 0) {
                panic!("Collinear lines!");
            } else {
                None
            }
        } else {
            None
        }
    } else {
        let q_minus_p_x_s = (&h2.pos - &h1.pos).cross_2d(&h2.vel);
        let t = (q_minus_p_x_s as f64) / (r_x_s as f64);
        let u = (q_minus_p_x_r as f64) / (r_x_s as f64);
        if (t < 0.0) || (u < 0.0) {
            None
        } else {
            Some((
                (h1.pos.x as f64) + t * (h1.vel.x as f64),
                (h1.pos.y as f64) + t * (h1.vel.y as f64),
            ))
        }
    }
}

fn parse_input(contents: &String) -> Vec<Hail> {
    contents
        .trim()
        .split('\n')
        .map(|line| {
            let (pos_str, vel_str) = line.split_once(" @ ").unwrap();
            let pos: Coord = pos_str.parse().unwrap();
            let vel: Coord = vel_str.parse().unwrap();
            Hail { pos, vel }
        })
        .collect()
}

fn compute_1(contents: &String) -> u64 {
    let hail_stones = parse_input(contents);
    let mut counter: u64 = 0;
    for i in 0..hail_stones.len() {
        let hail1 = &hail_stones[i];
        for j in (i + 1)..hail_stones.len() {
            let hail2 = &hail_stones[j];
            if hail1 != hail2 {
                if let Some(xy) = intersection_point_2d(hail1, hail2) {
                    if (TEST_AREA[0] <= xy.0)
                        && (xy.0 <= TEST_AREA[1])
                        && (TEST_AREA[0] <= xy.1)
                        && (xy.1 <= TEST_AREA[1])
                    {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}

// Assume WLOG that the first hailstone (defined by position/velocity p1 and v1)
// is at (0, 0) with zero velocity, and define all other hail stones and the rock
// relative to that hailstone.
//
// S    = starting rock position (relative to first hailstone)
// v    = rock velocity (relative to first hailstone)
// n{i} = time at which rock intersects with hail i
//
// int_1 = 0 = S + v * n1
// int_2 = p2 + v2 * n2 = S + v * n2 = v * (n2 - n1)
// int_3 = p3 + v3 * n3 = S + v * n3 = v * (n3 - n1)
//
// The intersection points must lie in a line, so we know that the
// cross product of any two intersection points is zero:
//
// (p2 + v2 * n2) x (p3 + v3 * n3) = 0
//
// https://en.wikipedia.org/wiki/Cross_product#Algebraic_properties
// (I had to remind myself on this...)
//
// (p2 x (p3 + v3 * n3)) + (v2 x (p3 + v3 * n3)) * n2 = 0
// (p2 x p3) + (p2 x v3) * n3 + (v2 x p3) * n2 + (v2 x v3) * n3 * n2 = 0
//
// Major trick: we need to get rid of the n3 * n2 term. Applying the dot
// product with v2 cancels any terms with a v2 x u for any u, and similarly
// applying the dot product with v3 cancels any terms with a v3 x u
// (because a x b is perpendicular to both a and b, and the dot product
// of perpendicular vectors is zero).
//
// (p2 x p3) • v2 + (p2 x v3) * n3 • v2 = 0
// (p2 x p3) • v3 + (v2 x p3) * n2 • v3 = 0
//
// n3 = (-(p2 x p3) • v2) / ((p2 x v3) • v2)
// n2 = (-(p2 x p3) • v3) / ((v2 x p3) • v3)
fn compute_2(contents: &String) -> i128 {
    let hail_stones = parse_input(contents);
    assert!(hail_stones.len() >= 3);
    let hail2 = &hail_stones[1] - &hail_stones[0];
    let hail3 = &hail_stones[2] - &hail_stones[0];

    // We're doing integer division here, so we're assuming that velocities/positions
    // of intersections are all integers as well. The description text kind
    // of implies that, though...
    let n3 = -(hail2.pos.cross(&hail3.pos).dot(&hail2.vel))
        / (hail2.pos.cross(&hail3.vel).dot(&hail2.vel));
    let n2 = -(hail2.pos.cross(&hail3.pos).dot(&hail3.vel))
        / (hail2.vel.cross(&hail3.pos).dot(&hail3.vel));

    let collision_point_2 = &hail_stones[1].pos + &(hail_stones[1].vel.scalar_mul(n2));
    let collision_point_3 = &hail_stones[2].pos + &(hail_stones[2].vel.scalar_mul(n3));
    let rock_velocity = (&collision_point_3 - &collision_point_2).scalar_div(n3 - n2);
    let rock_position = &collision_point_2 - &(rock_velocity.scalar_mul(n2));

    rock_position.x + rock_position.y + rock_position.z
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d24.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(31208, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(580043851566574, result);
    println!("part 2: {result}");
}
