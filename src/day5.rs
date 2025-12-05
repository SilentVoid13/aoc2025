use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<Range<usize>>, Vec<usize>);

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Input {
    let (p1, p2) = input.split_once("\n\n").unwrap();

    let mut ranges = Vec::new();
    for l in p1.lines() {
        let (v1, v2) = l.split_once('-').unwrap();
        let v1: usize = v1.parse().unwrap();
        let v2: usize = v2.parse().unwrap();
        ranges.push(v1..v2 + 1);
    }
    let mut available = Vec::new();
    for l in p2.lines() {
        let v: usize = l.parse().unwrap();
        available.push(v);
    }
    (ranges, available)
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> usize {
    let (ranges, available) = input;
    let mut res = 0;
    for val in available {
        for r in ranges {
            if r.contains(val) {
                res += 1;
                break;
            }
        }
    }
    res
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> usize {
    let (ranges, _) = input;
    let mut ranges = ranges.clone();
    let mut changed = true;
    while changed {
        changed = false;
        let mut new_ranges = Vec::new();
        for (i, r) in ranges.iter().cloned().enumerate() {
            let mut min_start = r.start;
            let mut max_end = r.end;
            for (i2, r2) in ranges.iter().cloned().enumerate() {
                if i == i2 {
                    continue;
                }
                if r.start < r2.end && r2.start < r.end {
                    changed = true;
                    min_start = min_start.min(r2.start);
                    max_end = max_end.max(r2.end);
                }
            }
            let new_r = min_start..max_end;
            if !new_ranges.contains(&new_r) {
                new_ranges.push(new_r);
            }
        }
        ranges = new_ranges;
    }

    let mut res = 0;
    for r in ranges {
        res += r.end - r.start;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 14);
    }
}
