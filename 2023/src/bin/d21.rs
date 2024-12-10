use std::{
    collections::{HashMap, HashSet},
    fs,
};

const MAP_DIMENSION: usize = 131;
const NUM_STEPS_PART_1: usize = 64;
const NUM_STEPS_PART_2: usize = 26501365;
// const NUM_STEPS_PART_2: usize = 65 + 131 * 4;

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

type Connections = HashMap<Location, Vec<Location>>;

fn parse_input(contents: &str) -> (Location, Connections) {
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

fn count_reachable(start: Location, connections: &Connections, num_steps: usize) -> usize {
    let mut currently_reachable: HashSet<&Location> = HashSet::new();
    currently_reachable.insert(&start);
    for _ in 0..num_steps {
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

// fn show_reachable(start: Location, connections: &Connections, num_steps: usize) -> () {
//     let mut currently_reachable: HashSet<&Location> = HashSet::new();
//     currently_reachable.insert(&start);
//     for _ in 0..num_steps {
//         let mut next_reachable = HashSet::new();
//         for loc in currently_reachable.iter() {
//             for connected_location in connections[loc].iter() {
//                 next_reachable.insert(connected_location);
//             }
//         }
//         currently_reachable = next_reachable;
//     }
//     let mut map = [[Tile::Garden; MAP_DIMENSION]; MAP_DIMENSION];
//     for loc in currently_reachable {
//         map[loc.x][loc.y] = Tile::Rock;
//     }
//     for line in map.iter() {
//         for tile in line.iter() {
//             match tile {
//                 Tile::Garden => {
//                     print!(" ");
//                 }
//                 Tile::Rock => {
//                     print!("X");
//                 }
//             }
//         }
//         println!("");
//     }
// }

fn compute_1(contents: &str) -> usize {
    let (start, connections) = parse_input(contents);
    count_reachable(start, &connections, NUM_STEPS_PART_1)
}

fn saturation_info(start: Location, connections: &Connections) -> (usize, usize, usize) {
    let mut currently_reachable: HashSet<&Location> = HashSet::new();
    currently_reachable.insert(&start);
    let mut last_four_counts: [usize; 4] = [1, 0, 0, 0];
    let mut i: usize = 0;

    while (last_four_counts[0] != last_four_counts[2])
        || (last_four_counts[1] != last_four_counts[3])
    {
        let mut next_reachable = HashSet::new();
        for loc in currently_reachable.iter() {
            for connected_location in connections[loc].iter() {
                next_reachable.insert(connected_location);
            }
        }
        currently_reachable = next_reachable;
        i += 1;
        last_four_counts[i % 4] = currently_reachable.len();
        // println!("{}, {:?}", i, last_four_counts);
    }
    // steps to saturate, # filled after even steps, # filled after odd steps
    (i - 2, last_four_counts[0], last_four_counts[1])
}

fn compute_2(contents: &str) -> usize {
    let (start, connections) = parse_input(contents);

    // We're gonna do a lot of cheating here.
    // The puzzle description leaves out several important assumptions, and worse,
    // the test input does NOT satisfy all of these important assumptions.
    // 1. There are direct, uninterrupted horizontal / vertical paths from
    //    the start to each of the four edges.
    // 2. There are no boulders along any of the four edges.
    // 3. The garden is a square with an odd length side
    // 4. You start in the center of the garden
    let steps_center_to_adjacent_edge = MAP_DIMENSION / 2 + 1; // even
    let steps_center_to_adjacent_corner_edge = MAP_DIMENSION + 1; // even
    let steps_edge_to_adjacent_edge = MAP_DIMENSION; // odd

    let upper_left = Location { x: 0, y: 0 };
    let lower_left = Location {
        x: 0,
        y: MAP_DIMENSION - 1,
    };
    let upper_right = Location {
        x: MAP_DIMENSION - 1,
        y: 0,
    };
    let lower_right = Location {
        x: MAP_DIMENSION - 1,
        y: MAP_DIMENSION - 1,
    };
    let center_upper = Location {
        x: MAP_DIMENSION / 2,
        y: 0,
    };
    let center_left = Location {
        x: 0,
        y: MAP_DIMENSION / 2,
    };
    let center_lower = Location {
        x: MAP_DIMENSION / 2,
        y: MAP_DIMENSION - 1,
    };
    let center_right = Location {
        x: MAP_DIMENSION - 1,
        y: MAP_DIMENSION / 2,
    };

    let dist_to_garden_plot = |x: i64, y: i64| -> usize {
        if (x == 0) && (y == 0) {
            0
        } else if (x == 0) || (y == 0) {
            steps_center_to_adjacent_edge
                + steps_edge_to_adjacent_edge * ((x.abs() + y.abs() - 1) as usize)
        } else {
            steps_center_to_adjacent_corner_edge
                + steps_edge_to_adjacent_edge * ((x.abs() + y.abs() - 2) as usize)
        }
    };

    // Could compute this by hand, but why bother.
    let mut x = 0;
    while dist_to_garden_plot(x, 0) <= NUM_STEPS_PART_2 {
        x += 1;
    }
    let x_hi = x;

    // COMPUTE THE COUNTS FOR GARDENS IN INTERIOR QUADRANT:
    //
    // xxx   xxx   xxx | xxx   xxx
    // xxx   xxx   xxx | xxx   xxx
    // xxx   xxx   xxx | xxx   xxx
    //                 |
    // xxx   xxx   xxx | xxx   xxx
    // xxx   xxx   xxx | xxx   xxx
    // xxx   xxx   xxx | xxx   xxx
    //                 +––––––––––
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xSx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    //
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    //
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx

    // The edge (not fully saturated) garden plots that are further away
    assert!(
        dist_to_garden_plot(x_hi - 1, 1) + saturation_info(lower_left, &connections).0
            > NUM_STEPS_PART_2
    );
    // The edge (not fully saturated) garden plots that are closer
    assert!(
        dist_to_garden_plot(x_hi - 2, 1) + saturation_info(lower_left, &connections).0
            > NUM_STEPS_PART_2
    );

    let num_far_edge_gardens_in_quadrant = (x_hi - 1) as usize;
    let num_close_edge_gardens_in_quadrant = (x_hi - 2) as usize;
    // It's the same distance to every edge in the interior of the quadrant
    let steps_to_reach_far_edge_garden = dist_to_garden_plot(x_hi - 1, 1);
    assert!(steps_to_reach_far_edge_garden <= NUM_STEPS_PART_2);
    let steps_to_reach_close_edge_garden = dist_to_garden_plot(x_hi - 2, 1);
    assert!(steps_to_reach_close_edge_garden <= NUM_STEPS_PART_2);

    let far_edge_counts: usize = [lower_left, lower_right, upper_left, upper_right]
        .map(|loc| {
            num_far_edge_gardens_in_quadrant
                * count_reachable(
                    loc,
                    &connections,
                    NUM_STEPS_PART_2 - steps_to_reach_far_edge_garden,
                )
        })
        .iter()
        .sum();
    let close_edge_counts: usize = [lower_left, lower_right, upper_left, upper_right]
        .map(|loc| {
            num_close_edge_gardens_in_quadrant
                * count_reachable(
                    loc,
                    &connections,
                    NUM_STEPS_PART_2 - steps_to_reach_close_edge_garden,
                )
        })
        .iter()
        .sum();
    // If we have an even # of steps left after reaching (1, 1), then we'll have
    // an even number of steps left after reaching (2n+1, 2m+1) for every integer n, m.
    // Vice versa as well.
    let x_hi_minus_three = x_hi - 3;
    let (num_even_parity_non_edges_in_quadrant, num_odd_parity_non_edges_in_quadrant) =
        if (NUM_STEPS_PART_2 - dist_to_garden_plot(1, 1)) % 2 == 0 {
            (
                (x_hi_minus_three / 2).pow(2),
                (x_hi_minus_three / 2) * (x_hi_minus_three / 2 + 1),
            )
        } else {
            (
                (x_hi_minus_three / 2) * (x_hi_minus_three / 2 + 1),
                (x_hi_minus_three / 2).pow(2),
            )
        };
    let non_edge_counts: usize = [lower_left, lower_right, upper_left, upper_right]
        .map(|loc| {
            let sat_info = saturation_info(loc, &connections);
            (num_even_parity_non_edges_in_quadrant as usize) * sat_info.1
                + (num_odd_parity_non_edges_in_quadrant as usize) * sat_info.2
        })
        .iter()
        .sum();
    let total_quadrant_reachables = far_edge_counts + close_edge_counts + non_edge_counts;

    // COMPUTE THE COUNTS FOR GARDENS ON DIRECT LINES:
    //
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    //
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    //                 +––––––––––
    // xxx   xxx   xxx | xxx   xxx
    // xxx   xxx   xSx | xxx   xxx
    // xxx   xxx   xxx | xxx   xxx
    //                 +––––––––––
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    //
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx
    // xxx   xxx   xxx   xxx   xxx

    assert!(dist_to_garden_plot(x_hi - 1, 0) < NUM_STEPS_PART_2);
    assert!(dist_to_garden_plot(x_hi, 0) > NUM_STEPS_PART_2);
    assert!(
        dist_to_garden_plot(x_hi - 1, 0) + saturation_info(center_left, &connections).0
            > NUM_STEPS_PART_2
    );
    assert!(
        dist_to_garden_plot(x_hi - 2, 0) + saturation_info(center_left, &connections).0
            <= NUM_STEPS_PART_2
    );

    let straight_away_edge_counts: usize = [center_left, center_right, center_lower, center_upper]
        .map(|loc| {
            count_reachable(
                loc,
                &connections,
                NUM_STEPS_PART_2 - dist_to_garden_plot(x_hi - 1, 0),
            )
        })
        .iter()
        .sum();
    // S E O E O E O T H
    // 0 1 2 3 4 5 6 7 8
    let (num_even_parity_straight_away_interiors, num_odd_parity_straight_away_interiors) =
        if (NUM_STEPS_PART_2 - dist_to_garden_plot(1, 0)) % 2 == 0 {
            (((x_hi - 1) / 2) as usize, ((x_hi - 2) / 2) as usize)
        } else {
            (((x_hi - 2) / 2) as usize, ((x_hi - 1) / 2) as usize)
        };
    let straight_away_interior_counts: usize =
        [center_left, center_right, center_lower, center_upper]
            .map(|loc| {
                let sat_info = saturation_info(loc, &connections);
                num_even_parity_straight_away_interiors * sat_info.1
                    + num_odd_parity_straight_away_interiors * sat_info.2
            })
            .iter()
            .sum();

    let total_straight_away_reachables = straight_away_edge_counts + straight_away_interior_counts;

    let center_garden_reachables = if (NUM_STEPS_PART_2 % 2) == 0 {
        saturation_info(start, &connections).1
    } else {
        saturation_info(start, &connections).2
    };

    total_quadrant_reachables + total_straight_away_reachables + center_garden_reachables
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d21.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(3773, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(625628021226274, result);
    println!("part 2: {result}");
}
