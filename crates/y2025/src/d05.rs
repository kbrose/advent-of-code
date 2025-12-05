use shared::Problem;

fn parse_input(contents: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut split = contents.trim().split("\n\n");
    let ranges = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut range = line.split('-');
            let lo = range.next().unwrap().parse().unwrap();
            let hi = range.next().unwrap().parse().unwrap();
            (lo, hi)
        })
        .collect();
    let products = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (ranges, products)
}

fn compute_1(contents: &str) -> u64 {
    let (ranges, products) = parse_input(contents);
    let mut count = 0;
    for product in products {
        for (lo, hi) in ranges.iter() {
            if lo <= &product && &product <= hi {
                count += 1;
                break;
            }
        }
    }
    count
}

fn compute_2(contents: &str) -> u64 {
    let (mut ranges, _) = parse_input(contents);

    // Sort lexicographically. This lets us iterate over the ranges just once.
    ranges.sort();

    let mut count = 0;
    let mut curr_lo = ranges[0].0;
    let mut curr_hi = ranges[0].1;
    for range in ranges[1..].into_iter() {
        // If the new range's low is higher than the current high, we're starting a new range.
        // Finish up the current one by adding to the running count, and reset curr_lo.
        if range.0 > curr_hi {
            count += curr_hi - curr_lo + 1;
            curr_lo = range.0;
        }

        // The high high value of the current range should always be the max possible value.
        curr_hi = std::cmp::max(curr_hi, range.1);
    }
    // Need to count the final range
    count += curr_hi - curr_lo + 1;

    count
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
        "643".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "342018167474526".to_string()
    }
}
