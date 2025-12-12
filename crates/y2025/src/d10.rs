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

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    free: Vec<usize>,
}

const EPSILON: f64 = 1e-10;

impl Matrix {
    fn new_from_machine(machine: Machine) -> Self {
        let mut data = vec![vec![0.0; machine.buttons.len() + 1]; machine.joltages.len()];
        for (i, button) in machine.buttons.iter().enumerate() {
            for j in button {
                data[*j][i] = 1.0;
            }
        }
        for (i, joltage) in machine.joltages.into_iter().enumerate() {
            data[i][machine.buttons.len()] = joltage as f64;
        }
        Self {
            rows: data.len(),
            cols: data[0].len(),
            data: data,
            free: vec![],
        }
    }

    #[allow(unused)]
    fn show(&self) {
        for row in self.data.iter() {
            print!("[ ");
            for col in row {
                print!("{: >5.1} ", col);
            }
            println!("]");
        }
        println!("");
    }

    // https://rosettacode.org/wiki/Reduced_row_echelon_form#Python
    fn reduced_row_echelon_form(&mut self) {
        let mut lead = 0;
        for r in 0..self.rows {
            if lead >= self.cols {
                return;
            }
            let mut i = r;
            while self.data[i][lead].abs() < EPSILON {
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
                    if lv.abs() >= EPSILON {
                        for j in 0..self.cols {
                            self.data[i][j] = self.data[i][j] - lv * self.data[r][j];
                        }
                    }
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
                pushes -= (*free_value as f64) * row[*coefficient_idx];
            }
            if pushes < -EPSILON {
                return None;
            }
            if (pushes - pushes.round()).abs() > EPSILON {
                return None;
            }
            num_pushes += pushes.round() as u64;
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
