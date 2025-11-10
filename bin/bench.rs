use std::{path::Path, time::Instant};

const DURATION_TO_BENCH_MS: u64 = 5000;
const BATCH_SIZE: usize = 50;

struct InputWithSolvers<P1, P2> {
    input: String,
    part_1_solver: P1,
    part_2_solver: P2,
}

impl<P1, P2> InputWithSolvers<P1, P2> {
    pub fn new(path: &Path, part_1_solver: P1, part_2_solver: P2) -> Self {
        let input = std::fs::read_to_string(path).unwrap();
        Self {
            input,
            part_1_solver,
            part_2_solver,
        }
    }
}
struct SolutionResult {
    solution: i64,
    mean_run_duration: std::time::Duration,
    run_count: usize,
}
fn run_bench(input: &str, solver: fn(&str) -> i64) -> SolutionResult {
    let solution = solver(input);
    let end_time = Instant::now() + std::time::Duration::from_millis(DURATION_TO_BENCH_MS);
    let start = Instant::now();
    let mut run_count = 0;
    while Instant::now() < end_time {
        run_count += BATCH_SIZE;
        for _ in 0..BATCH_SIZE {
            solver(input);
        }
    }
    let end = Instant::now();
    let mean_run_duration = (end - start) / run_count as u32;

    SolutionResult {
        solution,
        mean_run_duration,
        run_count,
    }
}

fn main() {
    type InputSolverPair = Vec<InputWithSolvers<fn(&str) -> i64, fn(&str) -> i64>>;
    let inputs: InputSolverPair = vec![InputWithSolvers::new(
        Path::new("data/day_01.txt"),
        aoc_2025_perf_lib::day_01::p1,
        aoc_2025_perf_lib::day_01::p2,
    )];

    let mut results: Vec<SolutionResult> = Vec::new();
    for InputWithSolvers {
        input,
        part_1_solver,
        part_2_solver,
    } in inputs
    {
        results.push(run_bench(&input, part_1_solver));
        results.push(run_bench(&input, part_2_solver));
    }

    for (
        day,
        SolutionResult {
            solution,
            mean_run_duration,
            run_count,
        },
    ) in results.into_iter().enumerate()
    {
        let day = day + 1;
        let part = ((day + 1) % 2) + 1;
        println!(
            "Day {day} part {part} [{}ns ({run_count} runs)] -> {solution}",
            mean_run_duration.as_nanos(),
        );
    }
}
