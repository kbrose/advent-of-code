use std::{collections::HashSet, fs};

struct MapRange {
    source: u64,
    dest: u64,
    range: u64,
}

struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn source_to_dest(&self, src: u64) -> u64 {
        for range in &self.ranges {
            if range.source <= src && src < range.source + range.range {
                return range.dest + (src - range.source);
            }
        }
        src
    }
    fn dest_to_source(&self, dst: u64) -> u64 {
        for range in &self.ranges {
            if range.dest <= dst && dst < range.dest + range.range {
                return range.source + (dst - range.dest);
            }
        }
        dst
    }
}

fn parse_input(contents: &String) -> (Vec<u64>, Vec<Map>) {
    let mut lines = contents.split('\n').filter(|s| s.len() > 0);
    let seeds = lines.next().unwrap()[7..]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut maps = vec![];
    let mut map_ranges = vec![];
    for line in lines.chain("terminate:".split_whitespace()) {
        if line.contains(':') {
            maps.push(Map { ranges: map_ranges });
            map_ranges = vec![];
        } else {
            let mut line_split = line.split_whitespace();
            let dest = line_split.next().unwrap().parse().unwrap();
            let source = line_split.next().unwrap().parse().unwrap();
            let range = line_split.next().unwrap().parse().unwrap();
            map_ranges.push(MapRange {
                dest,
                source,
                range,
            })
        }
    }
    (seeds, maps)
}

fn compute_1(contents: &String) -> u64 {
    let (seeds, maps) = parse_input(contents);
    let mut curr = u64::MAX;
    for mut seed in seeds {
        for map in maps.iter() {
            seed = map.source_to_dest(seed)
        }
        curr = std::cmp::min(curr, seed);
    }
    curr
}

fn compute_2(contents: &String) -> u64 {
    let (seeds, maps) = parse_input(contents);
    let seed_ranges: Vec<(u64, u64)> = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(a, b)| (*a, *b))
        .collect();
    // Find discontinuities, which are all of the critical points
    // of a bunch of piecewise strictly monotonically increasing functions
    // We want discontinuities in seed space, so work backwards.
    let mut prev_crit_points: HashSet<u64> = HashSet::new();
    prev_crit_points.extend(maps.last().unwrap().ranges.iter().map(|r| r.dest));
    prev_crit_points.extend(
        maps.last()
            .unwrap()
            .ranges
            .iter()
            .map(|r| r.dest + r.range + 1),
    );
    prev_crit_points.insert(0);
    let mut crit_points: HashSet<u64> = HashSet::new();
    for map in maps.iter().rev() {
        // crit_points is the set of critical points in the SOURCE space
        crit_points = HashSet::new();
        crit_points.insert(0);
        for range in &map.ranges {
            crit_points.insert(map.dest_to_source(range.dest));
            crit_points.insert(map.dest_to_source(range.dest + range.range + 1));
        }
        for prev_crit_point in prev_crit_points {
            crit_points.insert(map.dest_to_source(prev_crit_point));
        }
        prev_crit_points = crit_points.clone();
    }
    crit_points.extend(seed_ranges.iter().map(|(a, _)| a));
    crit_points = crit_points
        .into_iter()
        .filter(|c| {
            seed_ranges
                .iter()
                .any(|(start, range)| start <= c && *c < start + range)
        })
        .collect();
    let mut curr = u64::MAX;
    for mut seed in crit_points {
        for map in maps.iter() {
            seed = map.source_to_dest(seed)
        }
        curr = std::cmp::min(curr, seed);
    }
    curr
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d05.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(382895070, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(17729182, result);
    println!("part 2: {result}");
}
