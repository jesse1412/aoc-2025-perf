# Optimised rust AOC solutions
This repo contains my AOC 2025 solutions, which I'll be optimising over time.

# Goals
1. Solutions must be generic
2. Document optimisation steps/methods used
3. Incorporate low-level optimisations (e.g cache hits, branch prediction, SIMD, unrolling, etc)
4. IO speed is out of scope, so all benchmarks will cover a handler being passed a raw &str
5. **Minimise runtime!**

# Repo layout
This repo contains a number of binary crates (one for each part of each day), as well as a "bench" bin, which benchmarks the runtimes of all solutions. The bins for each part of each day use corresponding libs, and exist to make it easy to debug + bench/analyse code.

Additionally, a writeup of the optimisation process + notes is included in the lib for each day.

A `launch.json` has been included for VSCode users who would want to run this repo themselves for some reason.

# Useful profiling tools
`samply record target/release/bench`
`perf annotate --stdio --dsos=bench --symbol=day_01::part_2`
`perf stat -d ./target/release/bench`
`perf record -F 999 --call-graph dwarf  ./target/release/bench`