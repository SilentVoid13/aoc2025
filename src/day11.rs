use aoc_runner_derive::{aoc, aoc_generator};
use gxhash::{HashMap, HashMapExt};

type Input = (Vec<Vec<usize>>, HashMap<String, usize>);

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Input {
    let mut key_map: HashMap<String, usize> = HashMap::new();
    let mut h: HashMap<&str, Vec<&str>> = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        let (key, rest) = l.split_once(": ").unwrap();
        let vals: Vec<&str> = rest.split(' ').collect();
        h.entry(key).or_default().extend(vals);
        key_map.insert(key.to_string(), i);
    }
    // convert hashmap to adjacency list
    let mut res: Vec<Vec<usize>> = vec![Vec::new(); h.len()];
    for key in key_map.keys() {
        let i = key_map[key];
        for &v in &h[key.as_str()] {
            if v == "out" {
                res[i].push(usize::MAX);
            } else {
                res[i].push(key_map[v]);
            }
        }
    }
    (res, key_map)
}

pub fn top_down_dp(
    node: usize,
    vis_dac: bool,
    vis_fft: bool,
    adj_list: &[Vec<usize>],
    dac: usize,
    fft: usize,
    cache: &mut HashMap<(usize, bool, bool), usize>,
) -> usize {
    if let Some(&res) = cache.get(&(node, vis_dac, vis_fft)) {
        return res;
    }
    let vis_dac = node == dac || vis_dac;
    let vis_fft = node == fft || vis_fft;

    let mut total = 0;
    for &neighbor in &adj_list[node] {
        if neighbor == usize::MAX && vis_dac && vis_fft {
            total += 1;
        } else if neighbor != usize::MAX {
            total += top_down_dp(neighbor, vis_dac, vis_fft, adj_list, dac, fft, cache);
        }
    }
    cache.insert((node, vis_dac, vis_fft), total);
    total
}

#[aoc(day11, part1)]
pub fn part1(input: &Input) -> usize {
    let (adj_list, key_map) = input;
    let start = key_map["you"];
    top_down_dp(
        start,
        true,
        true,
        adj_list,
        usize::MAX,
        usize::MAX,
        &mut HashMap::default(),
    )
}

#[aoc(day11, part2)]
pub fn part2(input: &Input) -> usize {
    let (adj_list, key_map) = input;
    let svr = key_map["svr"];
    let dac = key_map["dac"];
    let fft = key_map["fft"];

    top_down_dp(
        svr,
        false,
        false,
        adj_list,
        dac,
        fft,
        &mut HashMap::default(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    const EXAMPLE2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE2)), 2);
    }
}
