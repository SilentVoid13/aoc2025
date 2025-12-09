use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::point::Point;

type Input = Vec<Point>;

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.lines() {
        let (x, y) = l.split_once(',').unwrap();
        let x: i64 = x.parse().unwrap();
        let y: i64 = y.parse().unwrap();
        res.push(Point::new(x, y));
    }
    res
}

#[aoc(day9, part1)]
pub fn part1(input: &Input) -> usize {
    let mut max_area = 0;
    for (i, p1) in input.iter().enumerate() {
        for p2 in input.iter().skip(i + 1) {
            let x = p1.x.abs_diff(p2.x) + 1;
            let y = p1.y.abs_diff(p2.y) + 1;
            let area = x * y;
            max_area = area.max(max_area);
        }
    }
    max_area as usize
}

fn rect_is_fully_contained(
    edges: &[(Point, Point)],
    rect_left: i64,
    rect_top: i64,
    rect_right: i64,
    rect_bottom: i64,
) -> bool {
    // for each edge, check if it overlaps with the rectangle
    // NOTE: this exploits the fact that all edges are axis-aligned
    for (edge_low, edge_high) in edges {
        // if x range overlap && y range overlap
        if rect_left < edge_high.x
            && rect_right > edge_low.x
            && rect_top < edge_high.y
            && rect_bottom > edge_low.y
        {
            return false;
        }
    }
    true
}

#[aoc(day9, part2)]
pub fn part2(input: &Input) -> usize {
    // part2 can roughly be generalized as the following:
    // given a set of points forming a polygon, we need to find if a given rectangle
    // fits completely inside the polygon

    // compute normalized edges for the polygon
    // horizontal = left to right
    // vertical = bottom to top
    let mut edges = Vec::with_capacity(input.len());
    for win in input.as_slice().windows(2) {
        let [p1, p2] = win else { unreachable!() };
        let e1 = Point::new(p1.x.min(p2.x), p1.y.min(p2.y));
        let e2 = Point::new(p1.x.max(p2.x), p1.y.max(p2.y));
        edges.push((e1, e2));
    }
    let last_p = input.last().unwrap();
    let first_p = input.first().unwrap();
    edges.push((
        Point::new(last_p.x.min(first_p.x), last_p.y.min(first_p.y)),
        Point::new(last_p.x.max(first_p.x), last_p.y.max(first_p.y)),
    ));

    let mut max_area = 0;
    for (i, p1) in input.iter().enumerate() {
        for p2 in input.iter().skip(i + 1) {
            let x = p1.x.abs_diff(p2.x) + 1;
            let y = p1.y.abs_diff(p2.y) + 1;
            let area = x * y;
            // optimization: skip if area is already less than max_area
            if area <= max_area {
                continue;
            }
            // skip lines
            if x == 1 || y == 1 {
                continue;
            }
            let rect_left = p1.x.min(p2.x);
            let rect_top = p1.y.min(p2.y);
            let rect_right = p1.x.max(p2.x);
            let rect_bottom = p1.y.max(p2.y);
            if rect_is_fully_contained(&edges, rect_left, rect_top, rect_right, rect_bottom) {
                max_area = max_area.max(area);
            }
        }
    }
    max_area as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 24);
    }
}
