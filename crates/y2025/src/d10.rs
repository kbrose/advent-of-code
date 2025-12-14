use shared::Problem;

type Lights = Vec<bool>;
type Buttons = Vec<Vec<usize>>;
type Joltages = Vec<u64>;

#[derive(Clone)]
struct Machine {
    lights: Lights,
    buttons: Buttons,
    joltages: Joltages,
}

fn parse_input(contents: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    for line in contents.trim().lines() {
        let (light_string, rest) = line.split_once(' ').unwrap();
        let lights = light_string
            .chars()
            .filter(|c| c == &'.' || c == &'#')
            .map(|c| c == '#')
            .collect::<Vec<bool>>();
        let (buttons_string, joltage_string) = rest.rsplit_once(' ').unwrap();
        let mut buttons: Vec<Vec<usize>> = buttons_string
            .split(' ')
            .map(|button_string| {
                button_string
                    .split(',')
                    .map(|mut s| {
                        if let Some(new_s) = s.strip_prefix('(') {
                            s = new_s;
                        }
                        if let Some(new_s) = s.strip_suffix(')') {
                            s = new_s;
                        }
                        s.parse::<usize>().unwrap()
                    })
                    .collect()
            })
            .collect();
        buttons.sort_by_key(|b| std::cmp::Reverse(b.len()));
        let joltages = joltage_string
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        machines.push(Machine {
            lights,
            buttons,
            joltages,
        });
    }
    machines
}

fn button_pushes_to_desired(buttons: &[Vec<usize>], curr: Lights, desired: &Lights) -> Option<u64> {
    let mut curr_min_pushes = None;
    if buttons.len() > 0 {
        for i in 0..buttons.len() {
            let mut new_curr = curr.clone();
            for j in buttons[i].iter() {
                new_curr[*j] = !new_curr[*j];
            }
            if &new_curr == desired {
                return Some(1);
            } else {
                if let Some(n) = button_pushes_to_desired(&buttons[i + 1..], new_curr, desired) {
                    match curr_min_pushes {
                        None => curr_min_pushes = Some(n + 1),
                        Some(curr_min) => curr_min_pushes = Some(std::cmp::min(n + 1, curr_min)),
                    }
                }
            }
        }
        curr_min_pushes
    } else {
        if &curr == desired { Some(0) } else { None }
    }
}

fn compute_1(contents: &str) -> u64 {
    let machines = parse_input(contents);
    let mut total = 0;
    for machine in machines {
        let desired = machine.lights;
        let buttons = machine.buttons;
        let push_count = button_pushes_to_desired(&buttons, vec![false; desired.len()], &desired)
            .expect("Unsolvable!");
        total += push_count;
    }
    total
}

mod frac {
    use std::{
        num::NonZeroU64,
        ops::{DivAssign, Mul, Sub, SubAssign},
    };

    // Note: since we're going to be guaranteeing a specific format
    // (reduced form, negative numerator if needed), then deriving
    // PartialEq and Eq are valid!
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Fraction {
        num: i64,
        den: NonZeroU64,
    }

    impl Fraction {
        pub fn new(num: i64, den: i64) -> Option<Fraction> {
            if den == 0 {
                None
            } else {
                let is_neg = (num < 0) ^ (den < 0);
                let mut num = num.unsigned_abs();
                let mut den = den.unsigned_abs();

                let gcd = compute_gcd(num, den);
                num /= gcd;
                den /= gcd;
                Some(Fraction {
                    num: if is_neg { -(num as i64) } else { num as i64 },
                    // unwrap is safe: den already checked for == 0
                    den: NonZeroU64::try_from(den).unwrap(),
                })
            }
        }

        pub fn new_from_int(num: i64) -> Fraction {
            Fraction {
                num,
                // unwrap is safe: den is non-zero
                den: NonZeroU64::try_from(1_u64).unwrap(),
            }
        }

        pub fn is_zero(&self) -> bool {
            self.num == 0
        }

        pub fn is_negative(&self) -> bool {
            self.num < 0
        }

        pub fn integral(&self) -> Option<i64> {
            if self.den == NonZeroU64::try_from(1_u64).unwrap() {
                Some(self.num)
            } else {
                None
            }
        }
    }

    impl Mul for Fraction {
        type Output = Fraction;

        fn mul(self, rhs: Self) -> Self::Output {
            Fraction::new(self.num * rhs.num, (self.den.get() * rhs.den.get()) as i64).unwrap()
        }
    }

    impl DivAssign for Fraction {
        fn div_assign(&mut self, rhs: Self) {
            // (a/b) / (c/d) = (a/b) * (d/c) = (ad)/(bc)
            if let Some(f) = Fraction::new(
                self.num * rhs.den.get() as i64,
                self.den.get() as i64 * rhs.num,
            ) {
                self.num = f.num;
                self.den = f.den;
            } else {
                panic!("Division by zero");
            }
        }
    }

    impl Sub for Fraction {
        type Output = Fraction;

        fn sub(self, rhs: Self) -> Self::Output {
            let lcm = compute_lcm(self.den.get(), rhs.den.get());
            let num1 = self.num * ((lcm / self.den.get()) as i64);
            let num2 = rhs.num * ((lcm / rhs.den.get()) as i64);
            Fraction::new(num1 - num2, lcm as i64).unwrap()
        }
    }

    impl SubAssign for Fraction {
        fn sub_assign(&mut self, rhs: Self) {
            let lcm = compute_lcm(self.den.get(), rhs.den.get());
            let num = self.num * ((lcm / self.den.get()) as i64)
                - rhs.num * ((lcm / rhs.den.get()) as i64);
            let den = lcm;
            let gcd = compute_gcd(num.unsigned_abs(), den);
            self.num = num / (gcd as i64);
            self.den = (den / gcd).try_into().unwrap();
        }
    }

    /// Implement Euclid's algorithm
    /// https://en.wikipedia.org/wiki/Euclidean_algorithm
    /// Props to 2023 day 8
    fn compute_gcd(a: u64, b: u64) -> u64 {
        if a == 0 && b == 0 {
            return 0;
        }
        let (mut a, mut b) = { if b < a { (b, a) } else { (a, b) } };
        while b != 0 {
            let prev = b;
            b = a % b;
            a = prev;
        }
        a
    }

    fn compute_lcm(a: u64, b: u64) -> u64 {
        if a == 0 && b == 0 {
            0
        } else {
            a / compute_gcd(a, b) * b
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let f = Fraction::new(0, 1).unwrap();
            assert_eq!(f.num, 0);
            assert_eq!(f.den, NonZeroU64::try_from(1).unwrap());

            let f = Fraction::new(2, 4).unwrap();
            assert_eq!(f.num, 1);
            assert_eq!(f.den, NonZeroU64::try_from(2).unwrap());

            let f = Fraction::new(-2, -4).unwrap();
            assert_eq!(f.num, 1);
            assert_eq!(f.den, NonZeroU64::try_from(2).unwrap());

            assert_eq!(Fraction::new(2, -4), Fraction::new(-1, 2));
            assert_eq!(Fraction::new(100, 200), Fraction::new(1, 2));
            assert_eq!(Fraction::new(-40, 4), Fraction::new(-10, 1));

            assert_eq!(Fraction::new(1, 0), None);
        }

        #[test]
        fn test_is_zero() {
            assert!(Fraction::new(0, 1).unwrap().is_zero());
            assert!(!Fraction::new(-1, 1).unwrap().is_zero());
            assert!(!Fraction::new(1, 1).unwrap().is_zero());
        }

        #[test]
        fn test_is_negative() {
            assert!(!Fraction::new(0, 1).unwrap().is_negative());
            assert!(Fraction::new(-1, 1).unwrap().is_negative());
            assert!(!Fraction::new(1, 1).unwrap().is_negative());
        }

        #[test]
        fn test_integral() {
            assert_eq!(Fraction::new(0, 1).unwrap().integral(), Some(0));
            assert_eq!(Fraction::new(-1, 1).unwrap().integral(), Some(-1));
            assert_eq!(Fraction::new(1, 1).unwrap().integral(), Some(1));
            assert_eq!(Fraction::new(1, 2).unwrap().integral(), None);
        }

        #[test]
        fn test_mul() {
            let f1 = Fraction::new(0, 1).unwrap();
            let f2 = Fraction::new(1, 100).unwrap();
            let f3 = Fraction::new(-1, 2).unwrap();
            let f4 = Fraction::new(1, 7).unwrap();
            let f5 = Fraction::new(10, 1).unwrap();

            for fx in [f1, f2, f3, f4] {
                for fy in [f1, f2, f3, f4] {
                    assert_eq!(fx * fy, fy * fx);
                }
            }

            assert_eq!(f1 * f2, Fraction::new_from_int(0));
            assert_eq!(f1 * f3, Fraction::new_from_int(0));

            assert_eq!(f2 * f3, Fraction::new(-1, 200).unwrap());

            assert_eq!(f2 * f2, Fraction::new(1, 10_000).unwrap());
            assert_eq!(f3 * f3, Fraction::new(1, 4).unwrap());

            assert_eq!(f2 * f4, Fraction::new(1, 700).unwrap());

            assert_eq!(f2 * f5, Fraction::new(1, 10).unwrap());
        }

        #[test]
        fn test_div_assign() {
            let f1 = Fraction::new(0, 1).unwrap();
            let f2 = Fraction::new(1, 100).unwrap();
            let f3 = Fraction::new(-1, 2).unwrap();
            let f4 = Fraction::new(1, 7).unwrap();
            let f5 = Fraction::new(10, 1).unwrap();

            let mut f = f1.clone();
            f /= f2;
            assert_eq!(f, f1);

            let mut f = f2.clone();
            f /= f2;
            assert_eq!(f, Fraction::new_from_int(1));

            let mut f = f3.clone();
            f /= f3;
            assert_eq!(f, Fraction::new_from_int(1));

            let mut f = f2.clone();
            f /= f3;
            assert_eq!(f, Fraction::new(-1, 50).unwrap());

            let mut f = f2.clone();
            f /= f4;
            assert_eq!(f, Fraction::new(7, 100).unwrap());

            let mut f = f2.clone();
            f /= f5;
            assert_eq!(f, Fraction::new(1, 1000).unwrap());
        }

        #[test]
        fn test_sub() {
            let f1 = Fraction::new(0, 1).unwrap();
            let f2 = Fraction::new(1, 100).unwrap();
            let f3 = Fraction::new(-1, 2).unwrap();
            let f4 = Fraction::new(1, 7).unwrap();
            let f5 = Fraction::new(10, 1).unwrap();

            assert_eq!(f1 - f2, Fraction::new(-1, 100).unwrap());
            assert_eq!(f2 - f1, f2);

            assert_eq!(f1 - f3, Fraction::new(1, 2).unwrap());
            assert_eq!(f2 - f3, Fraction::new(51, 100).unwrap());
            assert_eq!(f3 - f2, Fraction::new(-51, 100).unwrap());
            assert_eq!(f2 - f4, Fraction::new(-93, 700).unwrap());
            assert_eq!(f2 - f5, Fraction::new(-999, 100).unwrap());
            assert_eq!(
                Fraction::new(51, 100).unwrap() - Fraction::new(1, 100).unwrap(),
                Fraction::new(1, 2).unwrap()
            );
        }

        #[test]
        fn test_sub_assign() {
            let f1 = Fraction::new(0, 1).unwrap();
            let f2 = Fraction::new(1, 100).unwrap();
            let f3 = Fraction::new(-1, 2).unwrap();
            let f4 = Fraction::new(1, 7).unwrap();

            let mut f = f1.clone();
            f -= f2;
            assert_eq!(f, Fraction::new(-1, 100).unwrap());

            let mut f = f1.clone();
            f -= f3;
            assert_eq!(f, Fraction::new(1, 2).unwrap());

            let mut f = f2.clone();
            f -= f3;
            assert_eq!(f, Fraction::new(51, 100).unwrap());

            let mut f = f2.clone();
            f -= f4;
            assert_eq!(f, Fraction::new(-93, 700).unwrap());

            let mut f = Fraction::new_from_int(-10);
            f -= Fraction::new_from_int(12) * Fraction::new_from_int(0);
            assert_eq!(f, Fraction::new_from_int(-10));
        }
    }
}

use frac::Fraction;

struct Matrix {
    data: Vec<Vec<Fraction>>,
    rows: usize,
    cols: usize,
    free: Vec<usize>,
}

impl Matrix {
    fn new_from_machine(machine: Machine) -> Self {
        let mut data = vec![
            vec![Fraction::new_from_int(0); machine.buttons.len() + 1];
            machine.joltages.len()
        ];
        for (i, button) in machine.buttons.iter().enumerate() {
            for j in button {
                data[*j][i] = Fraction::new_from_int(1);
            }
        }
        for (i, joltage) in machine.joltages.into_iter().enumerate() {
            data[i][machine.buttons.len()] = Fraction::new_from_int(joltage as i64);
        }
        Self {
            rows: data.len(),
            cols: data[0].len(),
            data: data,
            free: vec![],
        }
    }

    // #[allow(unused)]
    // fn show(&self) {
    //     for row in self.data.iter() {
    //         print!("[ ");
    //         for col in row {
    //             print!("{: >5.1} ", col);
    //         }
    //         println!("]");
    //     }
    //     println!("");
    // }

    // https://rosettacode.org/wiki/Reduced_row_echelon_form#Python
    fn reduced_row_echelon_form(&mut self) {
        let mut lead = 0;
        for r in 0..self.rows {
            if lead >= self.cols {
                return;
            }
            let mut i = r;
            while self.data[i][lead].is_zero() {
                i += 1;
                if i == self.rows {
                    i = r;
                    lead += 1;
                    if lead == self.cols {
                        return;
                    }
                    // No non-zero values in the rest of the column means it's
                    // a free variable
                    self.free.push(lead - 1);
                }
            }
            self.data.swap(i, r);
            let lv = self.data[r][lead];
            for val in self.data[r].iter_mut() {
                *val /= lv;
            }
            for i in 0..self.rows {
                if i != r {
                    let lv = self.data[i][lead];
                    // This check is unnecessary
                    // if !lv.is_zero() {
                    for j in 0..self.cols {
                        self.data[i][j] = self.data[i][j] - lv * self.data[r][j];
                    }
                    // }
                }
            }
            lead += 1;
        }
        self.free.extend(lead..self.cols - 1);
    }

    fn check_solution_is_valid(&self, free_values: &[u64]) -> Option<u64> {
        let mut num_pushes = free_values.iter().sum();

        for row in self.data.iter() {
            // Try plugging in the given values for the free variables, and see if the
            // solution is consistent with our needs.
            let mut pushes = row[self.cols - 1];
            for (free_value, coefficient_idx) in free_values.iter().zip(self.free.iter()) {
                // print!(
                //     "{pushes:?} -= {:?} * {:?} --> ",
                //     Fraction::new_from_int(*free_value as i64),
                //     row[*coefficient_idx]
                // );
                pushes -= Fraction::new_from_int(*free_value as i64) * row[*coefficient_idx];
                // println!("{pushes:?}");
            }
            if pushes.is_negative() {
                return None;
            }
            if let Some(integer_push) = pushes.integral() {
                num_pushes += integer_push as u64;
            } else {
                return None;
            }
        }

        Some(num_pushes)
    }
}

fn part_2_recursion(
    matrix: &Matrix,
    free_values: &mut [u64],
    i: usize,
    max_required_pushes: u64,
    curr_min: &mut u64,
) {
    if i == free_values.len() {
        if let Some(n) = matrix.check_solution_is_valid(free_values) {
            if &n < curr_min {
                *curr_min = n;
            }
        }
    } else {
        for pushes in 0..max_required_pushes {
            free_values[i] = pushes;
            part_2_recursion(matrix, free_values, i + 1, max_required_pushes, curr_min);
        }
    }
}

fn solve_part_2(matrix: &Matrix, machine: Machine) -> u64 {
    let mut free_values = vec![0; matrix.free.len()];
    let mut min_pushes = u64::MAX;

    let max_required_pushes = *machine.joltages.iter().max().unwrap();

    part_2_recursion(
        matrix,
        &mut free_values,
        0,
        max_required_pushes,
        &mut min_pushes,
    );

    min_pushes
}

fn compute_2(contents: &str) -> u64 {
    let machines = parse_input(contents);
    let mut total = 0;
    for machine in machines.into_iter() {
        let mut matrix = Matrix::new_from_machine(machine.clone());
        matrix.reduced_row_echelon_form();
        let summand = solve_part_2(&matrix, machine);
        total += summand;
    }
    total
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
        "571".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "20869".to_string()
    }
}
