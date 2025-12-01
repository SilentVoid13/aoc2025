use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<isize>;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.lines() {
        let c = l.chars().next().unwrap();
        let v = l[1..].parse::<isize>().unwrap();
        if c == 'R' {
            res.push(v);
        } else {
            res.push(-v);
        }
    }
    res
}

#[aoc(day1, part1)]
pub fn part1(input: &Input) -> usize {
    let mut val = 50;
    let mut pass = 0;
    for v in input {
        val = (val + v).rem_euclid(100);
        if val == 0 {
            pass += 1;
        }
    }
    pass
}

#[aoc(day1, part2)]
pub fn part2(input: &Input) -> usize {
    let mut val = 50;
    let mut pass = 0;
    for v in input {
        let old_val = val;
        val += v;
        pass += (val / 100).unsigned_abs();
        if val <= 0 && old_val != 0 {
            pass += 1;
        }
        val = val.rem_euclid(100);
    }
    pass
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 6);
    }
}
