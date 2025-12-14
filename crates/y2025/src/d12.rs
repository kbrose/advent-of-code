use shared::Problem;

#[derive(PartialEq, Eq, Clone)]
struct Present {
    empties: Vec<(usize, usize)>,
}

impl Present {
    fn new(mut empties: Vec<(usize, usize)>) -> Self {
        empties.sort();
        Self { empties }
    }

    fn flip_horizontal(&self) -> Self {
        let mut empties = vec![];
        for (i, j) in &self.empties {
            empties.push((*i, 2 - *j));
        }
        Present::new(empties)
    }
    // fn flip_vertical(&self) -> Self {
    //     let mut empties = vec![];
    //     for (i, j) in &self.empties {
    //         empties.push((2 - *i, *j));
    //     }
    //     Present { empties }
    // }
    fn rotate_90_deg(&self) -> Self {
        let mut empties = vec![];
        for (i, j) in &self.empties {
            empties.push(match (*i, *j) {
                (0, 0) => (0, 2),
                (0, 1) => (1, 2),
                (0, 2) => (2, 2),
                (1, 0) => (0, 1),
                (1, 1) => (1, 1),
                (1, 2) => (2, 1),
                (2, 0) => (0, 0),
                (2, 1) => (1, 0),
                (2, 2) => (2, 0),
                _ => panic!("Uh oh"),
            });
        }
        Present::new(empties)
    }
    #[allow(unused)]
    fn show(&self) {
        for i in 0..3 {
            for j in 0..3 {
                if self.empties.contains(&(i, j)) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}

struct Grid {
    shape: (usize, usize),
    required_presents: Vec<u32>,
}

fn parse_input(contents: &str) -> (Vec<Vec<Present>>, Vec<Grid>) {
    let mut presents: Vec<Present> = vec![];
    let mut grids: Vec<Grid> = vec![];
    for text_group in contents.trim().split("\n\n") {
        if text_group.lines().next().unwrap().ends_with(':') {
            let mut empties = vec![];
            for (i, line) in text_group.lines().skip(1).enumerate() {
                for (j, c) in line.chars().enumerate() {
                    if c == '.' {
                        empties.push((i, j));
                    }
                }
            }
            presents.push(Present::new(empties));
        } else {
            for line in text_group.lines() {
                let (grid_shape_str, requirements_str) = line.split_once(": ").unwrap();
                let (m, n) = grid_shape_str.split_once('x').unwrap();
                let shape = (m.parse::<usize>().unwrap(), n.parse::<usize>().unwrap());
                let required_presents = requirements_str
                    .split(' ')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
                grids.push(Grid {
                    shape,
                    required_presents,
                })
            }
        }
    }
    let mut all_present_options = vec![];
    for mut present in presents {
        let mut options = vec![];
        let mut push_if_not_present = |new_present| {
            if !options.contains(&new_present) {
                options.push(new_present);
            }
        };

        for _ in 0..4 {
            push_if_not_present(present.clone());
            push_if_not_present(present.flip_horizontal());
            present = present.rotate_90_deg();
        }
        all_present_options.push(options);
    }
    (all_present_options, grids)
}

#[allow(unused)]
fn show_grid(grid: &Vec<Vec<bool>>) {
    for line in grid.iter() {
        for b in line.iter() {
            if *b {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn grid_is_satisfiable(
    present_options: &Vec<Vec<Present>>,
    present_areas: &Vec<usize>,
    required_presents: &mut Vec<u32>,
    grid: &mut Vec<Vec<bool>>,
    curr_i: usize,
    curr_j: usize,
) -> bool {
    // If we've placed all required presents, it is satisfiable
    if required_presents.iter().sum::<u32>() == 0 {
        return true;
    }
    // If we are extending past the grid, it's not satisfiable
    if curr_j + 2 >= grid[0].len() {
        return false;
    }
    // If the required area of presents we have left to place is more than the
    // area we have left to place in, it is not satisfiable
    let required_area = required_presents
        .iter()
        .zip(present_areas.iter())
        .map(|(required_count, area)| *required_count as usize * area)
        .sum::<usize>();
    let available_area = (grid[0].len() - (curr_j + 1)) * grid.len() + (grid.len() - curr_i);
    if required_area > available_area {
        return false;
    }
    if curr_i + 2 >= grid.len() {
        return grid_is_satisfiable(
            present_options,
            present_areas,
            required_presents,
            grid,
            0,
            curr_j + 3,
        );
    }
    for present_index in 0..required_presents.len() {
        if required_presents[present_index] > 0 {
            // It will never be optimal to NOT place a present in any given 3x3 square
            // ASSUMPTION: The bounding box of every present is a 3x3 square.
            for i_start in curr_i..curr_i + 3 {
                if i_start + 2 >= grid.len() {
                    break;
                }
                let j_start = curr_j;
                'present_loop: for present in present_options[present_index].iter() {
                    // Loop once to check if the present is placeable
                    for i in i_start..i_start + 3 {
                        for j in j_start..j_start + 3 {
                            if grid[i][j] && !present.empties.contains(&(i - i_start, j - j_start))
                            {
                                continue 'present_loop;
                            }
                        }
                    }
                    // Loop a second time to actually place the present
                    for i in i_start..i_start + 3 {
                        for j in j_start..j_start + 3 {
                            if !present.empties.contains(&(i - i_start, j - j_start)) {
                                grid[i][j] = true;
                            }
                        }
                    }
                    // show_grid(grid);
                    required_presents[present_index] -= 1;
                    if grid_is_satisfiable(
                        present_options,
                        present_areas,
                        required_presents,
                        grid,
                        i_start + 1,
                        curr_j,
                    ) {
                        return true;
                    }
                    // That was a dead end, reset back to state before placing present
                    required_presents[present_index] += 1;
                    for i in i_start..i_start + 3 {
                        for j in j_start..j_start + 3 {
                            if !present.empties.contains(&(i - i_start, j - j_start)) {
                                grid[i][j] = false;
                            }
                        }
                    }
                }
            }
        }
    }
    grid_is_satisfiable(
        present_options,
        present_areas,
        required_presents,
        grid,
        0,
        curr_j + 3,
    )
}

fn compute_1(contents: &str) -> u64 {
    let (present_options, grids) = parse_input(contents);
    let present_areas: Vec<usize> = present_options
        .iter()
        .map(|options| 9 - options[0].empties.len())
        .collect();
    let mut total = 0;
    for mut grid_info in grids.into_iter() {
        if grid_info.shape.0 / 3 * grid_info.shape.1 / 3
            >= grid_info
                .required_presents
                .iter()
                .map(|x| *x as usize)
                .sum::<usize>()
            && false
        {
            // Trivial case
            total += 1;
        } else {
            let mut grid = vec![vec![false; grid_info.shape.1]; grid_info.shape.0];
            if grid_is_satisfiable(
                &present_options,
                &present_areas,
                &mut grid_info.required_presents,
                &mut grid,
                0,
                0,
            ) {
                total += 1;
            }
        }
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
        "414".to_string()
    }
    fn solve2(&self, _: &str) -> String {
        "Merry Christmas!".to_string()
    }
    fn expected2(&self) -> String {
        "Merry Christmas!".to_string()
    }
}
