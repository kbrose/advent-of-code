use shared::Problem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Open,
}

#[derive(Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    i: usize,
    j: usize,
}

impl Point {
    fn shift(&self, dir: &Dir) -> Self {
        match dir {
            Dir::Up => Point {
                i: self.i - 1,
                j: self.j,
            },
            Dir::Right => Point {
                i: self.i,
                j: self.j + 1,
            },
            Dir::Down => Point {
                i: self.i + 1,
                j: self.j,
            },
            Dir::Left => Point {
                i: self.i,
                j: self.j - 1,
            },
        }
    }

    fn at<T>(&self, map: &[Vec<T>]) -> T
    where
        T: Copy,
    {
        map[self.i][self.j]
    }
}

struct Puzzle {
    map: Vec<Vec<Tile>>,
    location: Point,
    directions: Vec<Dir>,
}

fn parse_input(contents: &str) -> Puzzle {
    let (map_str, dir_str) = contents.trim().split_once("\n\n").unwrap();
    let mut start = Point { i: 0, j: 0 };
    let map = map_str
        .split('\n')
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Open,
                    'O' => Tile::Box,
                    '@' => {
                        start = Point { i, j };
                        Tile::Open
                    }
                    _ => {
                        panic!("Unexpected character {c}");
                    }
                })
                .collect()
        })
        .collect();
    let directions = dir_str
        .replace('\n', "")
        .chars()
        .map(|c| match c {
            '^' => Dir::Up,
            '>' => Dir::Right,
            'v' => Dir::Down,
            '<' => Dir::Left,
            _ => {
                panic!("Unexpected character {c}")
            }
        })
        .collect();
    Puzzle {
        map,
        location: start,
        directions,
    }
}

fn process_move(map: &mut [Vec<Tile>], location: Point, dir: Dir) -> Point {
    let new_location = location.shift(&dir);
    match new_location.at(map) {
        Tile::Wall => location,
        Tile::Box => {
            let mut query_point = new_location.shift(&dir);
            while query_point.at(map) == Tile::Box {
                query_point = query_point.shift(&dir);
            }
            match query_point.at(map) {
                Tile::Wall => location,
                Tile::Box => unreachable!(),
                Tile::Open => {
                    map[query_point.i][query_point.j] = Tile::Box;
                    map[new_location.i][new_location.j] = Tile::Open;
                    new_location
                }
            }
        }
        Tile::Open => new_location,
    }
}

fn gps_score(map: &[Vec<Tile>]) -> usize {
    let mut score = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if tile == &Tile::Box {
                score += i * 100 + j;
            }
        }
    }
    score
}

fn compute_1(contents: &str) -> usize {
    let puzzle = parse_input(contents);
    let mut map = puzzle.map;
    let mut location = puzzle.location;
    let directions = puzzle.directions;
    for dir in directions.into_iter() {
        location = process_move(&mut map, location, dir);
    }

    gps_score(&map)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile2 {
    Wall,
    BoxLeft,
    BoxRight,
    Open,
}

struct Puzzle2 {
    map: Vec<Vec<Tile2>>,
    location: Point,
    directions: Vec<Dir>,
}

fn parse_input2(contents: &str) -> Puzzle2 {
    let (map_str, dir_str) = contents.trim().split_once("\n\n").unwrap();
    let mut start = Point { i: 0, j: 0 };
    let map = map_str
        .split('\n')
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(j, c)| match c {
                    '#' => [Tile2::Wall, Tile2::Wall],
                    '.' => [Tile2::Open, Tile2::Open],
                    'O' => [Tile2::BoxLeft, Tile2::BoxRight],
                    '@' => {
                        start = Point { i, j: j * 2 };
                        [Tile2::Open, Tile2::Open]
                    }
                    _ => {
                        panic!("Unexpected character {c}");
                    }
                })
                .collect()
        })
        .collect();
    let directions = dir_str
        .replace('\n', "")
        .chars()
        .map(|c| match c {
            '^' => Dir::Up,
            '>' => Dir::Right,
            'v' => Dir::Down,
            '<' => Dir::Left,
            _ => {
                panic!("Unexpected character {c}")
            }
        })
        .collect();
    Puzzle2 {
        map,
        location: start,
        directions,
    }
}

fn process_move2(map: &mut [Vec<Tile2>], location: Point, dir: Dir) -> Point {
    let new_location = location.shift(&dir);
    match (new_location.at(map), &dir) {
        (Tile2::Wall, _) => location,
        (Tile2::BoxLeft | Tile2::BoxRight, Dir::Right | Dir::Left) => {
            let mut query_point = new_location.shift(&dir);
            while [Tile2::BoxLeft, Tile2::BoxRight].contains(&query_point.at(map)) {
                query_point = query_point.shift(&dir);
            }
            match query_point.at(map) {
                Tile2::Wall => location,
                Tile2::BoxLeft | Tile2::BoxRight => unreachable!(),
                Tile2::Open => {
                    map[new_location.i][new_location.j] = Tile2::Open;
                    let mut box_shift_point = new_location;
                    while box_shift_point != query_point {
                        box_shift_point = box_shift_point.shift(&dir);
                        map[box_shift_point.i][box_shift_point.j] =
                            match (box_shift_point.at(map), &dir) {
                                (Tile2::BoxLeft, _) => Tile2::BoxRight,
                                (Tile2::BoxRight, _) => Tile2::BoxLeft,
                                (Tile2::Open, Dir::Left) => Tile2::BoxLeft,
                                (Tile2::Open, Dir::Right) => Tile2::BoxRight,
                                _ => unreachable!(),
                            };
                    }
                    new_location
                }
            }
        }
        (Tile2::BoxLeft | Tile2::BoxRight, Dir::Up | Dir::Down) => {
            let mut robot_destination = location;
            let mut query_points_left = vec![];
            if new_location.at(map) == Tile2::BoxLeft {
                query_points_left.push(new_location);
            } else {
                query_points_left.push(new_location.shift(&Dir::Left));
            };
            let mut box_left_positions_to_shift: Vec<Point> = Vec::new();
            loop {
                if query_points_left.iter().any(|p| {
                    (p.at(map) == Tile2::Wall) || (p.shift(&Dir::Right).at(map) == Tile2::Wall)
                }) {
                    break;
                }
                if query_points_left.iter().all(|p| {
                    (p.at(map) == Tile2::Open) && (p.shift(&Dir::Right).at(map) == Tile2::Open)
                }) {
                    for box_pos in box_left_positions_to_shift.iter().rev() {
                        let new_box_pos = box_pos.shift(&dir);
                        map[new_box_pos.i][new_box_pos.j] = Tile2::BoxLeft;
                        map[new_box_pos.i][new_box_pos.j + 1] = Tile2::BoxRight;
                        map[box_pos.i][box_pos.j] = Tile2::Open;
                        map[box_pos.i][box_pos.j + 1] = Tile2::Open;
                    }
                    robot_destination = new_location;
                    break;
                }
                let mut new_query_points_left = Vec::with_capacity(query_points_left.len());
                for curr_qp in query_points_left.into_iter() {
                    box_left_positions_to_shift.push(curr_qp);
                    let new_qp = curr_qp.shift(&dir);
                    match new_qp.at(map) {
                        Tile2::Wall => new_query_points_left.push(new_qp),
                        Tile2::BoxLeft => new_query_points_left.push(new_qp),
                        Tile2::BoxRight => {
                            new_query_points_left.push(new_qp.shift(&Dir::Left));
                            let qp_shifted = new_qp.shift(&Dir::Right);
                            if [Tile2::BoxLeft, Tile2::Wall].contains(&qp_shifted.at(map)) {
                                new_query_points_left.push(qp_shifted);
                            }
                        }
                        Tile2::Open => {
                            let qp_shifted = new_qp.shift(&Dir::Right);
                            if [Tile2::BoxLeft, Tile2::Wall].contains(&qp_shifted.at(map)) {
                                new_query_points_left.push(qp_shifted);
                            }
                        }
                    }
                }
                query_points_left = new_query_points_left;
            }

            robot_destination
        }
        (Tile2::Open, _) => new_location,
    }
}

fn gps_score2(map: &[Vec<Tile2>]) -> usize {
    let mut score = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if tile == &Tile2::BoxLeft {
                score += i * 100 + j;
            }
        }
    }
    score
}

#[allow(dead_code)]
fn show(map: &[Vec<Tile2>], location: Point) {
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            let p = Point { i, j };
            if location == p {
                assert!(tile == &Tile2::Open);
                print!("@");
            } else {
                match tile {
                    Tile2::Wall => print!("#"),
                    Tile2::BoxLeft => print!("["),
                    Tile2::BoxRight => print!("]"),
                    Tile2::Open => print!("."),
                }
            }
        }
        println!();
    }
    println!();
}

fn compute_2(contents: &str) -> usize {
    let puzzle = parse_input2(contents);
    let mut map = puzzle.map;
    let mut location = puzzle.location;
    let directions = puzzle.directions;
    for dir in directions.into_iter() {
        // show(&map, location);
        location = process_move2(&mut map, location, dir);
    }
    // show(&map, location);

    gps_score2(&map)
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
        "1514333".to_string()
    }
    fn expected2(&self) -> String {
        "1528453".to_string()
    }
}
