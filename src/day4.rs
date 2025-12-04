use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{grid::Grid, point::Point};

type Input = Grid<u8>;

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Input {
    Grid::parse(input)
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> usize {
    let grid = input;
    let mut res = 0;
    for p in grid.iter() {
        if grid[p] != b'@' {
            continue;
        }
        let mut rolls = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                let p2 = p + Point::new(x, y);
                if grid.contains(p2) && grid[p2] == b'@' {
                    rolls += 1;
                }
            }
        }
        if rolls < 4 {
            res += 1;
        }
    }
    res
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> usize {
    let mut grid = input.clone();
    let mut res = 0;
    let mut removed = true;
    let mut to_remove = Vec::with_capacity(grid.bytes.len());
    while removed {
        removed = false;
        to_remove.clear();
        for p in grid.iter() {
            if grid[p] != b'@' {
                continue;
            }
            let mut rolls = 0;
            for x2 in -1..=1 {
                for y2 in -1..=1 {
                    if x2 == 0 && y2 == 0 {
                        continue;
                    }
                    let p2 = p + Point::new(x2, y2);
                    if grid.contains(p2) && grid[p2] == b'@' {
                        rolls += 1;
                    }
                }
            }
            if rolls < 4 {
                to_remove.push(p);
                res += 1;
            }
        }
        for p in &to_remove {
            grid[*p] = b'.';
            removed = true;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 43);
    }
}
