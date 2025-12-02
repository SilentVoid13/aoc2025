use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};
use gxhash::{HashSet, HashSetExt};

type Input = Vec<Range<usize>>;

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.split(",") {
        let (start, end) = l.split_once("-").unwrap();
        let r = start.parse::<usize>().unwrap()..end.parse::<usize>().unwrap() + 1;
        res.push(r);
    }
    res
}

pub fn _part1_bruteforce(input: &Input) -> usize {
    let mut res = 0;
    for range in input {
        for val in range.clone() {
            let num_digits = val.ilog10() + 1;
            if !num_digits.is_multiple_of(2) {
                continue;
            }
            let mid = num_digits / 2;
            let div = 10usize.pow(mid);
            let left = val / div;
            let right = val % div;
            if left == right {
                res += val;
            }
        }
    }
    res
}

pub fn _part2_bruteforce(input: &Input) -> usize {
    let mut res = 0;
    for range in input {
        for val in range.clone() {
            let num_digits = val.ilog10() + 1;
            for num_splits in 2..=num_digits {
                if !num_digits.is_multiple_of(num_splits) {
                    continue;
                }
                let part_size = num_digits / num_splits;
                let mut cur_val = val;
                let mut cur_part = usize::MAX;
                let mut valid = true;
                for i in 0..num_splits {
                    let div = 10usize.pow(part_size * (num_splits - 1 - i));
                    let part = cur_val / div;
                    cur_val %= div;
                    if cur_part == usize::MAX {
                        cur_part = part;
                    } else if cur_part != part {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    if val == 11 {
                        println!("{:?}", range);
                    }
                    res += val;
                    break;
                }
            }
        }
    }
    res
}

#[aoc(day2, part1)]
pub fn part1(input: &Input) -> usize {
    let mut res = 0;
    for range in input {
        res += find_duplicates(range.clone(), true);
    }
    res
}

#[aoc(day2, part2)]
pub fn part2(input: &Input) -> usize {
    let mut res = 0;
    for range in input {
        res += find_duplicates(range.clone(), false);
    }
    res
}

/// Homemade 'optimized' solution to find all numbers in the given range that consists of repeated parts.
/// For each start/end, we try all possible duplications
/// e.g. for 985-1200:
/// - 9-12:
///   - 9 => 999   (valid)
///   - 1 => 1111  (valid)
/// - 98-120:
///   - 98 => 9898
///   - 99 => 9999
///   - 10 => 1010 (valid)
///   - 11 => 1111 (valid)
///   - 12 => 1212
fn find_duplicates(r: Range<usize>, part1: bool) -> usize {
    let mut seen = HashSet::new();
    let d1 = r.start.ilog10() + 1;
    let d2 = r.end.ilog10() + 1;
    let mut res = 0;

    let part_sizes = if part1 {
        let middle = d1.div_ceil(2);
        middle..=middle
    } else {
        1..=(d1 / 2) + 1
    };

    for part_size in part_sizes {
        let start_part = r.start / 10usize.pow(d1 - part_size);
        let end_part = r.end / 10usize.pow(d1 - part_size);
        for mut part in start_part..=end_part {
            let mut cur_digits = part.ilog10() + 1;
            let mut n_chunks = (d1 / part_size) - 1;
            if cur_digits > part_size {
                part /= 10;
                cur_digits -= 1;
                n_chunks = (d2 / part_size) - 1;
                if part1 && n_chunks != 1 {
                    continue;
                }
            }
            let mut v = part;
            let mut multiplier = 1;
            let mul = 10usize.pow(cur_digits);
            for _ in 0..n_chunks {
                multiplier *= mul;
                v += part * multiplier;
            }

            if v != part && v >= r.start && v <= r.end && seen.insert(v) {
                res += v;
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1227775554);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 4174379265);
    }

    #[test]
    fn test_duplicates() {
        let r = 985..1201;
        assert_eq!(find_duplicates(r.clone(), true), 1010 + 1111);
        assert_eq!(find_duplicates(r.clone(), false), 999 + 1010 + 1111);
    }
}
