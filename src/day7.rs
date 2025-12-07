use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{
    grid::Grid,
    point::{DOWN, LEFT, Point, RIGHT},
};
use gxhash::{HashSet, HashSetExt};

type Input = (Grid<u8>, Point);

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    (grid, start)
}

#[aoc(day7, part1)]
pub fn part1(input: &Input) -> usize {
    let (grid, start) = input;
    let mut queue = vec![*start];
    let mut res = 0;
    let mut seen = Grid {
        width: grid.width,
        height: grid.height,
        bytes: vec![false; grid.width * grid.height],
    };
    seen[*start] = true;

    while let Some(pos) = queue.pop() {
        let mut down_pos = pos + DOWN;

        while grid.contains(down_pos) {
            if grid[down_pos] == b'^' {
                if !seen[down_pos] {
                    res += 1;
                }
                seen[down_pos] = true;

                let p1 = down_pos + LEFT;
                let p2 = down_pos + RIGHT;
                if !seen[p1] && grid.contains(p1) {
                    seen[p1] = true;
                    queue.push(p1);
                }
                if !seen[p2] && grid.contains(p2) {
                    seen[p2] = true;
                    queue.push(p2);
                }
                break;
            }
            seen[down_pos] = true;
            down_pos += DOWN;
        }
    }
    res
}

#[aoc(day7, part2)]
pub fn part2(input: &Input) -> usize {
    let (grid, start) = input;
    let mut queue = VecDeque::new();
    queue.push_back(*start);
    let mut seen = Grid {
        width: grid.width,
        height: grid.height,
        bytes: vec![false; grid.width * grid.height],
    };
    seen[*start] = true;

    let mut count = Grid {
        width: grid.width,
        height: grid.height,
        bytes: vec![0usize; grid.width * grid.height],
    };
    count[*start] = 1;

    while let Some(pos) = queue.pop_front() {
        let base_val = count[pos];
        let down_pos = pos + DOWN;
        if !grid.contains(down_pos) {
            continue;
        }

        // NOTE: we cannot use a while loop like in part1,
        // because we need to propagate counts correctly
        // we go 'level by level' in a BFS style
        if grid[down_pos] == b'^' {
            let p1 = down_pos + LEFT;
            let p2 = down_pos + RIGHT;
            count[p1] += base_val;
            count[p2] += base_val;
            if !seen[p1] {
                seen[p1] = true;
                queue.push_back(p1);
            }
            if !seen[p2] {
                seen[p2] = true;
                queue.push_back(p2);
            }
        } else {
            count[down_pos] += base_val;
            if !seen[down_pos] {
                queue.push_back(down_pos);
            }
            seen[down_pos] = true;
        }
    }
    let mut res = 0;
    for x in 0..grid.width {
        let p = Point::new(x as i64, (grid.height - 1) as i64);
        res += count[p];
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 40);
    }
}
