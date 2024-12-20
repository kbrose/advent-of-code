use std::fs;

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

fn compute_1(contents: &str) -> usize {
    let race = parse_input(contents);
    let mut curr_pos = race.start;
    let mut prev_pos = race.start;
    let mut curr_cost = 0;
    let mut cheat_counter = 0;
    while curr_pos != race.end {
        // First, iterate over the neighboring walls looking for cheats
        for next in curr_pos
            .neighbors()
            .into_iter()
            .filter(|next| (next != &prev_pos) && (next.at(&race.map) == Some(&Tile::Wall)))
        {
            for next_next in next.neighbors().into_iter() {
                if let Some(Tile::Track(cost_at_cheat_dest)) = next_next.at(&race.map) {
                    if *cost_at_cheat_dest > curr_cost + 2 + 99 {
                        cheat_counter += 1;
                    }
                }
            }
        }
        // Then, find the next "normal" move
        for neighbor in curr_pos
            .neighbors()
            .into_iter()
            .filter(|neighbor| (neighbor != &prev_pos))
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

fn compute_2(contents: &str) -> usize {
    let race = parse_input(contents);
    let mut curr_pos = race.start;
    let mut prev_pos = race.start;
    let mut curr_cost = 0;
    let mut cheat_counter = 0;
    // Pre-allocate this array, we're going to be reusing it inside the hot loop.
    let mut cheat_destinations: Vec<Point> = Vec::with_capacity(4);
    while curr_pos != race.end {
        // First, iterate over the possible cheats
        for i in 0..=20 {
            for j in 0..=(20 - i) {
                if i == 0 && j == 0 {
                    continue;
                }
                let cost_of_cheat = (i + j) as u32;
                cheat_destinations.clear();
                cheat_destinations.push(Point {
                    i: curr_pos.i + i,
                    j: curr_pos.j + j,
                });
                cheat_destinations.push(Point {
                    i: curr_pos.i.wrapping_sub(i),
                    j: curr_pos.j.wrapping_sub(j),
                });
                if i != 0 && j != 0 {
                    cheat_destinations.push(Point {
                        i: curr_pos.i + i,
                        j: curr_pos.j.wrapping_sub(j),
                    });
                    cheat_destinations.push(Point {
                        i: curr_pos.i.wrapping_sub(i),
                        j: curr_pos.j + j,
                    });
                }
                for point in cheat_destinations.iter() {
                    if let Some(Tile::Track(cost_at_cheat_dest)) = point.at(&race.map) {
                        if *cost_at_cheat_dest > curr_cost + cost_of_cheat + 99 {
                            cheat_counter += 1;
                        }
                    }
                }
            }
        }
        // Then, find the next "normal" move
        for neighbor in curr_pos
            .neighbors()
            .into_iter()
            .filter(|neighbor| (neighbor != &prev_pos))
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

fn main() {
    let contents =
        fs::read_to_string("inputs/d20.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(1307, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(986545, result);
    println!("part 2: {result}");
}
