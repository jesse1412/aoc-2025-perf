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
    let inputs: InputSolverPair = vec![
        InputWithSolvers::new(
            Path::new("my_data/day_01.txt"),
            aoc_2025_perf_lib::day_01::p1,
            aoc_2025_perf_lib::day_01::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_02.txt"),
            aoc_2025_perf_lib::day_02::p1,
            aoc_2025_perf_lib::day_02::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_03.txt"),
            aoc_2025_perf_lib::day_03::p1,
            aoc_2025_perf_lib::day_03::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_04.txt"),
            aoc_2025_perf_lib::day_04::p1,
            aoc_2025_perf_lib::day_04::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_05.txt"),
            aoc_2025_perf_lib::day_05::p1,
            aoc_2025_perf_lib::day_05::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_06.txt"),
            aoc_2025_perf_lib::day_06::p1,
            aoc_2025_perf_lib::day_06::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_07.txt"),
            aoc_2025_perf_lib::day_07::p1,
            aoc_2025_perf_lib::day_07::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_08.txt"),
            aoc_2025_perf_lib::day_08::p1,
            aoc_2025_perf_lib::day_08::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_09.txt"),
            aoc_2025_perf_lib::day_09::p1,
            aoc_2025_perf_lib::day_09::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_10.txt"),
            aoc_2025_perf_lib::day_10::p1,
            aoc_2025_perf_lib::day_10::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_11.txt"),
            aoc_2025_perf_lib::day_11::p1,
            aoc_2025_perf_lib::day_11::p2,
        ),
        InputWithSolvers::new(
            Path::new("my_data/day_12.txt"),
            aoc_2025_perf_lib::day_12::p1,
            aoc_2025_perf_lib::day_12::p2,
        ),
    ];

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

    let mut total_mean_runtime_ns = 0;
    let mut runtime_day_part_tuples = Vec::new();
    for (
        problem,
        SolutionResult {
            solution,
            mean_run_duration,
            run_count,
        },
    ) in results.into_iter().enumerate()
    {
        let day = problem / 2 + 1;
        let part = problem % 2 + 1;
        total_mean_runtime_ns += mean_run_duration.as_nanos();
        runtime_day_part_tuples.push((mean_run_duration.as_nanos(), day, part));
        println!(
            "Day {day} part {part} [{}ns ({run_count} runs)] -> {solution}",
            mean_run_duration.as_nanos(),
        );
    }
    println!(
        "Mean runtime: {total_mean_runtime_ns}ns ({}s)",
        total_mean_runtime_ns / 10_u128.pow(9)
    );
    println!("Sorted run times:");
    runtime_day_part_tuples.sort_unstable();
    for (runtime_ns, day, part) in runtime_day_part_tuples {
        println!("Day {day} part {part}: {runtime_ns}ns");
    }
}
