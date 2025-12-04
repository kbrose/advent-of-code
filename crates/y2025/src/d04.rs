use shared::Problem;

mod grid {
    pub struct Grid {
        elements: Vec<bool>,
        num_rows: usize,
        num_cols: usize,
    }

    impl Grid {
        pub fn new(grid: Vec<Vec<bool>>) -> Grid {
            let num_rows = grid.len();
            let num_cols = grid.first().map_or(0, |v| v.len());
            assert!(grid.iter().all(|v| v.len() == num_cols));
            let elements = grid.into_iter().flatten().collect();
            Grid {
                elements,
                num_rows,
                num_cols,
            }
        }

        fn get(&self, i: usize, j: usize) -> Option<bool> {
            if i >= self.num_cols || j >= self.num_rows {
                None
            } else {
                Some(self.elements[i * self.num_cols + j])
            }
        }

        fn remove(&mut self, i: usize, j: usize) -> bool {
            if i >= self.num_cols || j >= self.num_rows {
                false
            } else {
                self.elements[i * self.num_cols + j] = false;
                true
            }
        }

        pub fn is_accessible(&self, i: usize, j: usize) -> bool {
            let mut count = 0;
            for (a, b) in [
                (i.wrapping_sub(1), j.wrapping_sub(1)),
                (i.wrapping_sub(1), j),
                (i.wrapping_sub(1), j + 1),
                (i, j.wrapping_sub(1)),
                (i, j + 1),
                (i + 1, j.wrapping_sub(1)),
                (i + 1, j),
                (i + 1, j + 1),
            ] {
                if let Some(true) = self.get(a, b) {
                    count += 1
                }
            }
            count < 4
        }

        pub fn count_accessible(&self) -> u64 {
            let mut count = 0;
            for i in 0..self.num_cols {
                for j in 0..self.num_rows {
                    if let Some(true) = self.get(i, j) {
                        if self.is_accessible(i, j) {
                            count += 1;
                        }
                    }
                }
            }
            count
        }

        pub fn count_accessible_2(&mut self) -> u64 {
            let mut count = 0;
            let mut prev_count = 1;
            while count != prev_count {
                prev_count = count;
                for i in 0..self.num_cols {
                    for j in 0..self.num_rows {
                        if let Some(true) = self.get(i, j) {
                            if self.is_accessible(i, j) {
                                count += 1;
                                self.remove(i, j);
                            }
                        }
                    }
                }
            }
            count
        }
    }
}

use grid::Grid;

fn parse_input(contents: &str) -> Grid {
    Grid::new(
        contents
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| c == '@').collect())
            .collect(),
    )
}

fn compute_1(contents: &str) -> u64 {
    let grid = parse_input(contents);
    grid.count_accessible()
}

fn compute_2(contents: &str) -> u64 {
    let mut grid = parse_input(contents);
    grid.count_accessible_2()
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
        "1441".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "9050".to_string()
    }
}
