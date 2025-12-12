# aoc2025

runs part1 + part2 for all days combined in ~274ms, only single-threaded on a laptop with an i5-1135G7:

```txt
$ hyperfine --warmup 3 --runs 100 target/release/aoc2025

Benchmark 1: target/release/aoc2025
  Time (mean ± σ):     274.1 ms ±  10.6 ms    [User: 261.6 ms, System: 11.1 ms]
  Range (min … max):   267.5 ms … 356.8 ms    100 runs
```

the majority of that time is taken by day10 part2 (~235ms), which requires z3 (or you have to write your own linalg solver, which feels more like math than programming tbh).
