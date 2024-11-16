use std::{
    collections::{HashMap, HashSet},
    fs,
};

const MAP_DIMENSION: usize = 131;
const NUM_STEPS: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Garden,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

fn parse_input(contents: &String) -> (Location, HashMap<Location, Vec<Location>>) {
    let mut map = [[Tile::Garden; MAP_DIMENSION]; MAP_DIMENSION];
    let mut start = Location { x: 0, y: 0 };
    for (x, line) in contents.trim().split('\n').enumerate() {
        for (y, character) in line.chars().enumerate() {
            match character {
                '.' => {} // All tiles defaulted to garden anyway...
                '#' => {
                    map[x][y] = Tile::Rock;
                }
                'S' => {
                    start = Location { x, y };
                }
                _ => panic!("Unexpected character!"),
            }
        }
    }
    let mut connections: HashMap<Location, Vec<Location>> = HashMap::new();
    for x in 0..MAP_DIMENSION {
        for y in 0..MAP_DIMENSION {
            if map[x][y] == Tile::Rock {
                continue;
            }
            let mut conns: Vec<Location> = Vec::with_capacity(4);

            if (x > 0) && (map[x - 1][y] == Tile::Garden) {
                conns.push(Location { x: x - 1, y });
            }
            if (x < MAP_DIMENSION - 1) && (map[x + 1][y] == Tile::Garden) {
                conns.push(Location { x: x + 1, y });
            }
            if (y > 0) && (map[x][y - 1] == Tile::Garden) {
                conns.push(Location { x, y: y - 1 });
            }
            if (y < MAP_DIMENSION - 1) && (map[x][y + 1] == Tile::Garden) {
                conns.push(Location { x, y: y + 1 });
            }
            conns.shrink_to_fit();
            connections.insert(Location { x, y }, conns);
        }
    }
    (start, connections)
}

fn compute_1(contents: &String) -> usize {
    let (start, connections) = parse_input(contents);
    let mut currently_reachable: HashSet<&Location> = HashSet::new();
    currently_reachable.insert(&start);
    for _ in 0..NUM_STEPS {
        let mut next_reachable = HashSet::new();
        for loc in currently_reachable.iter() {
            for connected_location in connections[loc].iter() {
                next_reachable.insert(connected_location);
            }
        }
        currently_reachable = next_reachable;
    }
    currently_reachable.len()
}

fn compute_2(_contents: &String) -> u64 {
    todo!()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d21.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    // assert_eq!(670984704, result);
    println!("part 1: {result}");

    // let result = compute_2(&contents);
    // assert_eq!(262775362119547, result);
    // println!("part 2: {result}");
}
