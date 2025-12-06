use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{grid::Grid, point::Point};

type Input = String;

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Input {
    input.to_string()
}

fn compute_ops(cols: &[Vec<usize>], ops: &[char]) -> usize {
    let mut res = 0;
    for (col, op) in cols.iter().zip(ops.iter()) {
        let mut r = col[0];
        for v in &col[1..] {
            match *op {
                '*' => {
                    r *= v;
                }
                '+' => {
                    r += v;
                }
                _ => {}
            }
        }
        res += r;
    }
    res
}

#[aoc(day6, part1)]
pub fn part1(input: &Input) -> usize {
    let num_cols = input.lines().next().unwrap().split_whitespace().count();
    let mut cols = vec![Vec::new(); num_cols];
    let mut ops = Vec::new();
    let num_lines = input.lines().count();

    for l in input.lines().take(num_lines - 1) {
        for (i, val) in l.split_whitespace().enumerate() {
            let v = val.parse::<usize>().unwrap();
            cols[i].push(v);
        }
    }
    for op in input.lines().last().unwrap().split_whitespace() {
        ops.push(op.chars().next().unwrap());
    }
    compute_ops(&cols, &ops)
}

#[aoc(day6, part2)]
pub fn part2(input: &Input) -> usize {
    let mut cols = Vec::new();
    let mut ops = Vec::new();
    for op in input.lines().last().unwrap().split_whitespace() {
        ops.push(op.chars().next().unwrap());
    }
    let grid = Grid::parse(input);

    let mut vals = Vec::new();
    let mut cur_col = Vec::new();
    for x in 0..grid.width {
        vals.clear();
        for y in 0..grid.height - 1 {
            let p = Point::new(x as i64, y as i64);
            let v = grid[p];
            if v == b' ' {
                continue;
            }
            vals.push(v - b'0');
        }
        let mut real_v = 0;
        for (i, v) in vals.iter().rev().enumerate() {
            real_v += 10usize.pow(i as u32) * *v as usize;
        }
        // 0 == new column
        if real_v == 0 {
            cols.push(cur_col.clone());
            cur_col.clear();
        } else {
            cur_col.push(real_v);
        }
    }
    cols.push(cur_col);

    compute_ops(&cols, &ops)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 4277556);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 3263827);
    }
}
