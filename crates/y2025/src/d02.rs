use std::collections::HashSet;

use shared::Problem;

fn parse_input(contents: &str) -> Vec<(u64, u64)> {
    contents
        .trim()
        .split(',')
        .map(|product_range| {
            let mut split = product_range.split('-');
            let lo = split.next().unwrap();
            let hi = split.next().unwrap();
            (lo.parse::<u64>().unwrap(), hi.parse::<u64>().unwrap())
        })
        .collect()
}

fn num_digits(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

fn next_power_of_10(n: u64) -> u64 {
    if n == 0 { 1 } else { 10_u64.pow(num_digits(n)) }
}

/// Assumes lo and hi have the same number of digits!
fn sum_invalid_part_1(lo: u64, hi: u64) -> u64 {
    assert_eq!(num_digits(lo), num_digits(hi));

    let digits = num_digits(lo);

    if digits % 2 == 0 {
        let mut base_start = lo / (10_u64.pow(digits / 2));
        if base_start * (10_u64.pow(digits / 2)) + base_start < lo {
            base_start += 1;
        }
        let mut base_end = hi / (10_u64.pow(digits / 2));
        if base_end * (10_u64.pow(digits / 2)) + base_end > hi {
            base_end -= 1;
        }
        // Closed form solution for the sum of the numbers:
        // We want to sum a sequence like
        //  123123 + 124124 + 125125 + ... + 234234
        // First, note that this is equivalent to
        //       123 +    124 +    125 + ... +    234
        //  + 123000 + 124000 + 125000 + ... + 234000
        //  = 123 + 124 + 125 + ... + 234 + 1000 * (123 + 124 + 125 + ... + 234)
        // So if N is equal to 123 + 124 + 125 + ... + 234, then the total sum
        // is N + N * 1000.
        // The sum of numbers between a and b (aka N) can be derived by taking the
        // b'th triangular number minus the (a-1)'th triangular number.
        // b * (b + 1)   (a - 1) * a   b * (b + 1) - (a - 1) * a
        // ––––––––––– - ––––––––––– = –––––––––––––––––––––––––
        //     2              2                   2
        // Note that base_start is guaranteed to be > 0 since we don't allow preceding 0s.
        let out = (base_end * (base_end + 1) - (base_start - 1) * base_start) / 2;
        out + out * (10_u64.pow(digits / 2))
    } else {
        0
    }
}

fn compute_1(contents: &str) -> u64 {
    let ranges = parse_input(contents);
    let mut summand = 0;
    for (mut lo, final_hi) in ranges {
        // We're going to split up the range into parts that have the same numbers of digits.
        // This will simplify the logic of finding repeats.
        let mut hi = std::cmp::min(final_hi, next_power_of_10(lo) - 1);
        summand += sum_invalid_part_1(lo, hi);
        while hi < final_hi {
            lo = hi + 1;
            hi = std::cmp::min(final_hi, next_power_of_10(lo) - 1);
            summand += sum_invalid_part_1(lo, hi);
        }
    }
    summand
}

fn create_full_number_from_base(n: u64, repeats: u32) -> u64 {
    let digits = num_digits(n);
    let mut out = 0;
    for _ in 0..repeats {
        out *= 10_u64.pow(digits);
        out += n;
    }
    out
}

/// Assumes lo and hi have the same number of digits!
fn sum_invalid_part_2(lo: u64, hi: u64) -> u64 {
    assert_eq!(num_digits(lo), num_digits(hi));

    let digits_in_lo = num_digits(lo);

    let mut observed_numbers: HashSet<u64> = HashSet::new();

    for num_repeats in 2..=digits_in_lo {
        if digits_in_lo % num_repeats == 0 {
            let digits_in_repeat = digits_in_lo / num_repeats;
            let mut base_start = lo / (10_u64.pow(digits_in_repeat).pow(num_repeats - 1));
            let mut base_end = hi / (10_u64.pow(digits_in_repeat).pow(num_repeats - 1));
            if create_full_number_from_base(base_start, num_repeats) < lo {
                base_start += 1;
            }
            if create_full_number_from_base(base_end, num_repeats) > hi {
                base_end -= 1;
            }
            for i in base_start..=base_end {
                // Need to use a HashSet here because we can end up with repeats.
                // E.g. "2222" can show up with num_repeats = 2 AND num_repeats = 4
                // That wasn't the case for part 1 since num_repeats must equal 2 there.
                observed_numbers.insert(create_full_number_from_base(i, num_repeats));
            }
        }
    }

    observed_numbers.iter().sum::<u64>()
}

fn compute_2(contents: &str) -> u64 {
    let ranges = parse_input(contents);
    let mut summand = 0;
    for (mut lo, final_hi) in ranges {
        let mut hi = std::cmp::min(final_hi, next_power_of_10(lo) - 1);
        summand += sum_invalid_part_2(lo, hi);
        while hi < final_hi {
            lo = hi + 1;
            hi = std::cmp::min(final_hi, next_power_of_10(lo) - 1);
            summand += sum_invalid_part_2(lo, hi);
        }
    }
    summand
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
        "20223751480".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected2(&self) -> String {
        "30260171216".to_string()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1-2,3-4";
        let expected = vec![(1, 2), (3, 4)];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0), 1);
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(9), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(99), 2);
        assert_eq!(num_digits(100), 3);
    }

    #[test]
    fn test_next_power_of_10() {
        assert_eq!(next_power_of_10(0), 1);
        assert_eq!(next_power_of_10(1), 10);
        assert_eq!(next_power_of_10(9), 10);
        assert_eq!(next_power_of_10(10), 100);
        assert_eq!(next_power_of_10(99), 100);
        assert_eq!(next_power_of_10(100), 1000);
    }

    #[test]
    fn test_count_invalid_part_1() {
        assert_eq!(sum_invalid_part_1(11, 22), 33);
        assert_eq!(sum_invalid_part_1(1188511880, 1188511890), 1188511885);
        assert_eq!(sum_invalid_part_1(446443, 446449), 446446);
    }

    #[test]
    fn test_count_invalid_part_2() {
        assert_eq!(sum_invalid_part_2(11, 33), 66);
        assert_eq!(sum_invalid_part_2(1188511880, 1188511890), 1188511885);
        assert_eq!(sum_invalid_part_2(446443, 446449), 446446);
        assert_eq!(sum_invalid_part_2(565653, 565659), 565656);
        assert_eq!(sum_invalid_part_2(100, 115), 111);
        assert_eq!(sum_invalid_part_2(998, 999), 999);
        assert_eq!(sum_invalid_part_2(1000, 1012), 1010);
        assert_eq!(sum_invalid_part_2(565653, 565659), 565656);
        assert_eq!(sum_invalid_part_2(222220, 222224), 222222);
    }
}
