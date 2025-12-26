# Optimised rust AOC solutions
This repo contains my AOC 2025 solutions, which I'll be optimising over time.

# Goals
1. Solutions must be generic
2. Document optimisation steps/methods used
3. Incorporate low-level optimisations (e.g cache hits, branch prediction, SIMD, unrolling, etc)
4. IO speed is out of scope, so all benchmarks will cover a handler being passed a raw &str
5. **Minimise runtime!**

# Initial bench results
These were the bench results for my first iteration of solutions. Total runtime (mean) 295010449ns (0.295s).

Sorted by day/part:
```
Day 1 part 1 [50633ns (98750 runs)] -> 1048
Day 1 part 2 [52019ns (96150 runs)] -> 6498
Day 2 part 1 [71306137ns (100 runs)] -> 18893502033
Day 2 part 2 [120693280ns (50 runs)] -> 26202168557
Day 3 part 1 [17471ns (286200 runs)] -> 16812
Day 3 part 2 [124943ns (40050 runs)] -> 166345822896410
Day 4 part 1 [84167ns (59450 runs)] -> 1356
Day 4 part 2 [365020ns (13750 runs)] -> 8713
Day 5 part 1 [28169ns (177500 runs)] -> 577
Day 5 part 2 [6992ns (715050 runs)] -> 350513176552950
Day 6 part 1 [35890ns (139350 runs)] -> 5346286649122
Day 6 part 2 [92987ns (53800 runs)] -> 10389131401929
Day 7 part 1 [42049ns (118950 runs)] -> 1543
Day 7 part 2 [55809ns (89600 runs)] -> 3223365367809
Day 8 part 1 [7867903ns (650 runs)] -> 12
Day 8 part 2 [68139353ns (100 runs)] -> 3767453340
Day 9 part 1 [18453ns (271000 runs)] -> 4750092396
Day 9 part 2 [9473441ns (550 runs)] -> 1468516555
Day 10 part 1 [1847968ns (2750 runs)] -> 500
Day 10 part 2 [14407994ns (350 runs)] -> 19763
Day 11 part 1 [81837ns (61100 runs)] -> 714
Day 11 part 2 [161871ns (30900 runs)] -> 333852915427200
Day 12 part 1 [56062ns (89200 runs)] -> 495
Day 12 part 2 [1ns (7856581650 runs)] -> 0
```

Sorted run times:
```
Day 12 part 2: 1ns
Day 5 part 2: 6992ns
Day 3 part 1: 17471ns
Day 9 part 1: 18453ns
Day 5 part 1: 28169ns
Day 6 part 1: 35890ns
Day 7 part 1: 42049ns
Day 1 part 1: 50633ns
Day 1 part 2: 52019ns
Day 7 part 2: 55809ns
Day 12 part 1: 56062ns
Day 11 part 1: 81837ns
Day 4 part 1: 84167ns
Day 6 part 2: 92987ns
Day 3 part 2: 124943ns
Day 11 part 2: 161871ns
Day 4 part 2: 365020ns
Day 10 part 1: 1847968ns
Day 8 part 1: 7867903ns
Day 9 part 2: 9473441ns
Day 10 part 2: 14407994ns
Day 8 part 2: 68139353ns
Day 2 part 1: 71306137ns
Day 2 part 2: 120693280ns
```

# Repo layout
This repo contains a number of binary crates (one for each part of each day), as well as a "bench" bin, which benchmarks the runtimes of all solutions. The bins for each part of each day use corresponding libs, and exist to make it easy to debug + bench/analyse code.

Additionally, a writeup of the optimisation process + notes is included in the lib for each day.

A `launch.json` has been included for VSCode users who would want to run this repo themselves for some reason.

# Useful profiling tools
`samply record target/release/bench`
`perf annotate --stdio --dsos=bench --symbol=day_01::part_2`
`perf stat -d ./target/release/bench`
`perf record -F 999 --call-graph dwarf  ./target/release/bench`