use shared::Problem;

#[derive(Debug, Clone)]
enum Dir {
    L,
    R,
    U,
    D,
}

#[derive(Debug, Clone)]
struct Beam {
    row: usize,
    col: usize,
    dir: Dir,
    is_entry: bool,
}

impl Beam {
    fn march(&mut self, num_rows: usize, num_cols: usize) -> bool {
        if self.is_entry {
            self.is_entry = false;
            return true;
        }
        match self.dir {
            Dir::L => {
                if self.col == 0 {
                    return false;
                } else {
                    self.col -= 1
                }
            }
            Dir::R => {
                if self.col == num_cols - 1 {
                    return false;
                } else {
                    self.col += 1
                }
            }
            Dir::U => {
                if self.row == 0 {
                    return false;
                } else {
                    self.row -= 1
                }
            }
            Dir::D => {
                if self.row == num_rows - 1 {
                    return false;
                } else {
                    self.row += 1
                }
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
enum Device {
    UpRightMirror,
    DownRightMirror,
    HorizontalSplitter,
    VerticalSplitter,
    Empty,
}

#[derive(Debug, Clone)]
struct Tile {
    device: Device,
    from_left: bool,
    from_right: bool,
    from_up: bool,
    from_bottom: bool,
}

impl Tile {
    fn new(device: Device) -> Self {
        Tile {
            device,
            from_left: false,
            from_up: false,
            from_right: false,
            from_bottom: false,
        }
    }

    fn is_energized(&self) -> bool {
        self.from_left || self.from_right || self.from_up || self.from_bottom
    }
}

type Contraption = (Vec<Vec<Tile>>, usize, usize);

fn parse_input(contents: &str) -> Contraption {
    let contraption: Vec<Vec<Tile>> = contents
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '/' => Tile::new(Device::UpRightMirror),
                    '\\' => Tile::new(Device::DownRightMirror),
                    '-' => Tile::new(Device::HorizontalSplitter),
                    '|' => Tile::new(Device::VerticalSplitter),
                    '.' => Tile::new(Device::Empty),
                    _ => panic!("Unexpected character {c}"),
                })
                .collect()
        })
        .collect();
    let num_rows = contraption.len();
    let num_cols = contraption[0].len();
    (contraption, num_rows, num_cols)
}

fn compute(contraption: Contraption, initial_beam: Option<Beam>) -> u64 {
    let (mut contraption, num_rows, num_cols) = contraption;
    let mut beams = vec![match initial_beam {
        Some(beam) => beam,
        None => Beam {
            row: 0,
            col: 0,
            dir: Dir::R,
            is_entry: true,
        },
    }];
    while let Some(mut beam) = beams.pop() {
        if beam.march(num_rows, num_cols) {
            // contraption[beam.row][beam.col];
            match beam.dir {
                Dir::L => {
                    if !contraption[beam.row][beam.col].from_right {
                        contraption[beam.row][beam.col].from_right = true;
                        match contraption[beam.row][beam.col].device {
                            Device::UpRightMirror => {
                                beam.dir = Dir::D;
                                beams.push(beam);
                            }
                            Device::DownRightMirror => {
                                beam.dir = Dir::U;
                                beams.push(beam);
                            }
                            Device::VerticalSplitter => {
                                beams.push(Beam {
                                    row: beam.row,
                                    col: beam.col,
                                    dir: Dir::U,
                                    is_entry: false,
                                });
                                beams.push(Beam {
                                    row: beam.row,
                                    col: beam.col,
                                    dir: Dir::D,
                                    is_entry: false,
                                });
                            }
                            _ => beams.push(beam),
                        }
                    }
                }
                Dir::R => {
                    if !contraption[beam.row][beam.col].from_left {
                        contraption[beam.row][beam.col].from_left = true;
                        match contraption[beam.row][beam.col].device {
                            Device::UpRightMirror => {
                                beam.dir = Dir::U;
                                beams.push(beam);
                            }
                            Device::DownRightMirror => {
                                beam.dir = Dir::D;
                                beams.push(beam);
                            }
                            Device::VerticalSplitter => {
                                beams.push(Beam {
                                    row: beam.row,
                                    col: beam.col,
                                    dir: Dir::D,
                                    is_entry: false,
                                });
                                beams.push(Beam {
                                    row: beam.row,
                                    col: beam.col,
                                    dir: Dir::U,
                                    is_entry: false,
                                });
                            }
                            _ => beams.push(beam),
                        }
                    }
                }
                Dir::U => {
                    if !contraption[beam.row][beam.col].from_bottom {
                        contraption[beam.row][beam.col].from_bottom = true;
                        match contraption[beam.row][beam.col].device {
                            Device::UpRightMirror => {
                                beam.dir = Dir::R;
                                beams.push(beam);
                            }
                            Device::DownRightMirror => {
                                beam.dir = Dir::L;
                                beams.push(beam);
                            }
                            Device::HorizontalSplitter => {
                                beams.push(Beam {
                                    row: beam.row,
                                    col: beam.col,
                                    dir: Dir::L,
                                    is_entry: false,
                                });
                                beams.push(Beam {
                                    row: beam.row,
                                    col: beam.col,
                                    dir: Dir::R,
                                    is_entry: false,
                                });
                            }
                            _ => beams.push(beam),
                        }
                    }
                }
                Dir::D => {
                    if !contraption[beam.row][beam.col].from_up {
                        contraption[beam.row][beam.col].from_up = true;
                        match contraption[beam.row][beam.col].device {
                            Device::UpRightMirror => {
                                beam.dir = Dir::L;
                                beams.push(beam);
                            }
                            Device::DownRightMirror => {
                                beam.dir = Dir::R;
                                beams.push(beam);
                            }
                            Device::HorizontalSplitter => {
                                beams.push(Beam {
                                    row: beam.row,
                                    col: beam.col,
                                    dir: Dir::L,
                                    is_entry: false,
                                });
                                beams.push(Beam {
                                    row: beam.row,
                                    col: beam.col,
                                    dir: Dir::R,
                                    is_entry: false,
                                });
                            }
                            _ => beams.push(beam),
                        }
                    }
                }
            }
        }
    }
    contraption
        .iter()
        .map(|row| row.iter().map(|t| t.is_energized() as u64).sum::<u64>())
        .sum()
}

fn compute_1(contents: &str) -> u64 {
    let contraption = parse_input(contents);
    compute(contraption, None)
}

fn compute_2(contents: &str) -> u64 {
    let (contraption, num_rows, num_cols) = parse_input(contents);
    (0..num_rows)
        .flat_map(|row| {
            vec![
                Beam {
                    row,
                    col: 0,
                    dir: Dir::R,
                    is_entry: true,
                },
                Beam {
                    row,
                    col: num_cols - 1,
                    dir: Dir::L,
                    is_entry: true,
                },
            ]
        })
        .chain((0..num_cols).flat_map(|col| {
            vec![
                Beam {
                    row: 0,
                    col,
                    dir: Dir::D,
                    is_entry: true,
                },
                Beam {
                    row: num_rows - 1,
                    col,
                    dir: Dir::U,
                    is_entry: true,
                },
            ]
        }))
        .map(|beam| compute((contraption.clone(), num_rows, num_cols), Some(beam)))
        .max()
        .unwrap()
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
        "7046".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "7313".to_string()
    }
}
