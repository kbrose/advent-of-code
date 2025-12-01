use shared::Problem;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Track(u32),
    Wall,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    i: usize,
    j: usize,
}

impl Point {
    fn neighbors(&self) -> [Point; 4] {
        [
            Point {
                i: self.i.wrapping_sub(1),
                j: self.j,
            },
            Point {
                i: self.i + 1,
                j: self.j,
            },
            Point {
                i: self.i,
                j: self.j.wrapping_sub(1),
            },
            Point {
                i: self.i,
                j: self.j + 1,
            },
        ]
    }

    fn at<'a>(&self, map: &'a [Vec<Tile>]) -> Option<&'a Tile> {
        map.get(self.i).and_then(|map_row| map_row.get(self.j))
    }
}

struct Race {
    map: Vec<Vec<Tile>>,
    start: Point,
    end: Point,
}

fn parse_input(contents: &str) -> Race {
    let mut start = Point {
        i: usize::MAX,
        j: usize::MAX,
    };
    let mut end = start;
    let mut map: Vec<Vec<Tile>> = contents
        .trim()
        .split('\n')
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => {
                        start = Point { i, j };
                        Tile::Track(0)
                    }
                    'E' => {
                        end = Point { i, j };
                        Tile::Track(0)
                    }
                    '.' => Tile::Track(0),
                    '#' => Tile::Wall,
                    _ => panic!("Unexpected character {c}"),
                })
                .collect()
        })
        .collect();
    let mut curr_pos = start;
    let mut prev_pos = start;
    let mut dist: u32 = 0;
    while curr_pos != end {
        dist += 1;
        for neighbor in curr_pos.neighbors().into_iter() {
            if neighbor == prev_pos {
                continue;
            }
            if let Some(Tile::Track(_)) = neighbor.at(&map) {
                prev_pos = curr_pos;
                curr_pos = neighbor;
                map[curr_pos.i][curr_pos.j] = Tile::Track(dist);
                break;
            }
        }
    }
    Race { map, start, end }
}

#[allow(dead_code)]
fn show(race: &Race, cheat_1: &Point, cheat_2: &Point) {
    for (i, row) in race.map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            let point = &(Point { i, j });
            if point == cheat_1 {
                print!("1");
            } else if point == cheat_2 {
                print!("2");
            } else {
                match tile {
                    Tile::Track(_) => print!("."),
                    Tile::Wall => print!("#"),
                }
            }
        }
        println!();
    }
    println!();
}

fn add_usize_i32(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u.wrapping_sub(i.unsigned_abs() as usize)
    } else {
        u + (i as usize)
    }
}

fn count_desired_cheats(race: Race, cheat_step_len: i32) -> u64 {
    let mut curr_pos = race.start;
    let mut prev_pos = race.start;
    let mut curr_cost = 0;
    let mut cheat_counter = 0;
    while curr_pos != race.end {
        // First, iterate over the possible cheats
        for i in -cheat_step_len..=cheat_step_len {
            for j in -(cheat_step_len - i.abs())..=(cheat_step_len - i.abs()) {
                let cost_of_cheat = i.unsigned_abs() + j.unsigned_abs();
                let point = Point {
                    i: add_usize_i32(curr_pos.i, i),
                    j: add_usize_i32(curr_pos.j, j),
                };

                if let Some(Tile::Track(cost_at_cheat_dest)) = point.at(&race.map) {
                    if *cost_at_cheat_dest > curr_cost + cost_of_cheat + 99 {
                        cheat_counter += 1;
                    }
                }
            }
        }
        // Then, find the next "normal" move
        for neighbor in curr_pos
            .neighbors()
            .into_iter()
            .filter(|neighbor| neighbor != &prev_pos)
        {
            if let Some(Tile::Track(c)) = neighbor.at(&race.map) {
                prev_pos = curr_pos;
                curr_pos = neighbor;
                curr_cost = *c;
                break;
            };
        }
    }
    cheat_counter
}

fn compute_1(contents: &str) -> u64 {
    let race = parse_input(contents);
    count_desired_cheats(race, 2)
}

fn compute_2(contents: &str) -> u64 {
    let race = parse_input(contents);
    count_desired_cheats(race, 20)
}

pub(crate) struct Day {}

impl Problem for Day {
    fn source_code_file(&self) -> String {
        file!().to_string()
    }
    fn solve1(&self, contents: &str) -> String {
        format!("{}", compute_1(contents))
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected1(&self) -> String {
        "1307".to_string()
    }
    fn expected2(&self) -> String {
        "986545".to_string()
    }
}
