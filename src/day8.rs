use aoc_runner_derive::{aoc, aoc_generator};

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }

    pub fn euclidean_distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        dx * dx + dy * dy + dz * dz
    }
}

// wrapper to implement Ord for f64
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct F64Ord(pub f64);

impl Eq for F64Ord {}

impl Ord for F64Ord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct DisjointSet {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>,
    pub components: usize,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        let parent = (0..size).collect();
        let rank = vec![0; size];
        Self {
            parent,
            rank,
            components: size,
        }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
            self.components -= 1;
        }
    }
}

type Input = Vec<Point>;

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Input {
    let mut res = Vec::new();
    for l in input.lines() {
        let xyz: Vec<i32> = l.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        res.push(Point::new(xyz[0], xyz[1], xyz[2]));
    }
    res
}

fn build_min_pairs(input: &Input) -> BinaryHeap<Reverse<(F64Ord, usize, usize)>> {
    let mut heap: BinaryHeap<Reverse<(F64Ord, usize, usize)>> = BinaryHeap::new();
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let p1 = input[i];
            let p2 = input[j];
            let dist = p1.euclidean_distance(&p2);
            heap.push(Reverse((F64Ord(dist), i, j)));
        }
    }
    heap
}

#[aoc(day8, part1)]
pub fn part1(input: &Input) -> usize {
    part1_testable(input, 1000)
}

pub fn part1_testable(input: &Input, n: usize) -> usize {
    let mut heap = build_min_pairs(input);
    let mut diset = DisjointSet::new(input.len());
    for _ in 0..n {
        let entry = heap.pop().unwrap();
        let (_, p1, p2) = entry.0;
        // already connected
        if diset.find(p1) != diset.find(p2) {
            diset.union(p1, p2);
        }
    }

    let n = diset.parent.len();
    let mut counts = vec![0usize; n];
    for i in 0..n {
        let root = diset.find(i);
        counts[root] += 1;
    }
    counts.sort_unstable_by(|a, b| b.cmp(a));
    let mut res = 1;
    for size in counts.into_iter().take(3) {
        res *= size;
    }
    res
}

#[aoc(day8, part2)]
pub fn part2(input: &Input) -> usize {
    let mut heap = build_min_pairs(input);
    let mut diset = DisjointSet::new(input.len());
    let mut last_pair = None;
    while diset.components > 1 {
        let entry = heap.pop().unwrap();
        let (_, p1, p2) = entry.0;

        // already connected
        if diset.find(p1) != diset.find(p2) {
            diset.union(p1, p2);
            last_pair = Some((p1, p2));
        }
    }
    let (p1, p2) = last_pair.unwrap();
    input[p1].x as usize * input[p2].x as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1_testable(&parse(EXAMPLE), 10), 40);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 25272);
    }
}
