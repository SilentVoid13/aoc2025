use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use gxhash::HashSet;
use z3::{
    Optimize, SatResult,
    ast::{self, Int},
};

type Input = Vec<(usize, Vec<Vec<usize>>, Vec<usize>)>;

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.lines() {
        let (state_chars, rest) = &l[1..].split_once(']').unwrap();
        let mut final_state = 0;
        for (ci, c) in state_chars.chars().enumerate() {
            if c == '#' {
                final_state ^= 1 << ci;
            }
        }
        let mut buttons = Vec::new();
        for s in rest.split('(').skip(1) {
            let mut button = Vec::new();
            let (button_vals, _) = s.split_once(')').unwrap();
            for v in button_vals.split(',') {
                button.push(v.parse::<usize>().unwrap());
            }
            buttons.push(button);
        }
        let (_, rest) = rest.split_once('{').unwrap();
        let rest = &rest[..rest.len() - 1];
        let mut joltage = Vec::new();
        for s in rest.split(',') {
            let v = s.parse::<usize>().unwrap();
            joltage.push(v);
        }

        res.push((final_state, buttons, joltage));
    }
    res
}

#[aoc(day10, part1)]
pub fn part1(input: &Input) -> usize {
    let mut res = 0;
    for inp in input {
        let (final_state, buttons, _) = inp;
        let mut q = VecDeque::new();
        let mut visited = HashSet::default();
        let mut best = None;
        q.push_back((0, 0));
        // simple BFS to find the first time we reach final_state
        while let Some((state, num_press)) = q.pop_front() {
            if state == *final_state {
                best = Some(num_press);
                break;
            }
            for button in buttons {
                let mut new_state = state;
                for &b in button {
                    new_state ^= 1 << b;
                }
                if visited.insert(new_state) {
                    q.push_back((new_state, num_press + 1));
                }
            }
        }
        res += best.unwrap();
    }
    res
}

#[aoc(day10, part2)]
pub fn part2(input: &Input) -> usize {
    let mut res = 0;
    for inp in input {
        let (_, buttons, joltage) = inp;
        let solver = Optimize::new();

        let mut press_vars = Vec::new();
        for i in 0..buttons.len() {
            let var = ast::Int::new_const(format!("press{}", i));
            solver.assert(&var.ge(0));
            press_vars.push(var);
        }

        for (i, jolt_val) in joltage.iter().enumerate() {
            let mut sum_jolt = Vec::new();
            for j in 0..buttons.len() {
                let button = &buttons[j];
                if !button.contains(&i) {
                    continue;
                }
                let press = &press_vars[j];
                sum_jolt.push(press);
            }
            solver.assert(&Int::add(&sum_jolt).eq(*jolt_val as i64));
        }

        let sum_press = Int::add(&press_vars);
        solver.minimize(&sum_press);

        if solver.check(&[]) == SatResult::Sat {
            let model = solver.get_model().unwrap();
            let sum = model.eval(&sum_press, false).unwrap().as_u64().unwrap();
            res += sum as usize;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 33);
    }
}
