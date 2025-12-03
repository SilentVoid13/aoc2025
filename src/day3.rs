use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<u8>>;

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.lines() {
        let mut r = Vec::new();
        for c in l.bytes() {
            r.push(c - b'0');
        }
        res.push(r);
    }
    res
}

#[aoc(day3, part1)]
pub fn part1(input: &Input) -> usize {
    let mut res = 0;
    for row in input {
        res += find_best_joltage(row, 2);
    }
    res
}

#[aoc(day3, part2)]
pub fn part2(input: &Input) -> usize {
    let mut res = 0;
    for row in input {
        res += find_best_joltage(row, 12);
    }
    res
}

fn find_best_joltage(mut row: &[u8], size: usize) -> usize {
    let mut vals = Vec::new();
    for i in 0..size {
        let mut first = 0;
        let mut first_i = 0;
        let row_no_last = &row[..row.len() - (size - i - 1)];
        for (i, &v) in row_no_last.iter().enumerate() {
            if v > first {
                first = v;
                first_i = i;
            }
        }
        row = &row[first_i + 1..];
        vals.push(first);
    }

    let mut res = 0;
    for (i, v) in vals.iter().rev().enumerate() {
        res += usize::from(*v) * 10usize.pow(i as u32);
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 357);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 3121910778619);
    }
}
