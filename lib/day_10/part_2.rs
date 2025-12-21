use good_lp::{SolverModel, constraint, variable, variables};

pub fn run(input: &str) -> i64 {
    let res = {
        let mut res = 0;
        for line in input.lines() {
            res += handle_line(line);
        }
        res
    } as i64;
    res
}

pub fn handle_line(line: &str) -> u64 {
    let line_split_vec: Vec<&str> = line.split(' ').collect();
    let target_joltages: Vec<f64> = line_split_vec
        .last()
        .unwrap()
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(',')
        .map(|n_string| n_string.parse::<f64>().unwrap())
        .collect();

    let mut button_matrix = vec![vec![0.0; line_split_vec.len() - 2]; target_joltages.len()];
    for (i, button) in line_split_vec[1..line_split_vec.len() - 1]
        .into_iter()
        .enumerate()
    {
        for button_joltage_pos in button
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(',')
        {
            let button_joltage_pos: usize = button_joltage_pos.parse::<usize>().unwrap();
            button_matrix[button_joltage_pos][i] = 1.0;
        }
    }

    let mut vars = variables!();
    let solution_vars: Vec<_> = (0..button_matrix[0].len())
        .map(|_| vars.add(variable().min(0.0).integer()))
        .collect();

    let sum_of_vars = solution_vars.iter().sum::<good_lp::Expression>();
    let mut model = vars
        .minimise(sum_of_vars)
        .using(good_lp::solvers::microlp::microlp);

    for (r, row) in button_matrix.iter().enumerate() {
        let lhs: good_lp::Expression = row
            .iter()
            .zip(&solution_vars)
            .map(|(coeff, var)| *var * *coeff)
            .sum();

        model = model.with(constraint!(lhs == target_joltages[r]));
    }
    let solution = model.solve().unwrap();

    let obj = solution.into_inner().objective();
    let res = obj.round() as u64;
    res
}

// First example in math:
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// Can be represented as:
// m * (d)
//     + n * (b + d)
//     + o * c
//     + p * (c + d)
//     + q * (a + c)
//     + r * (a + b)
// = 3a + 5b + 4c + 7d

// Rewrite first equation:
// md + nb + nd + oc + pc + pd + qa + qc + ra + rb = 3a + 5b +4c + 7d
// a(q + r) + b(n + r) + c(o + p + q) + d(m + n + p) = 3a + 5b + 4c + 7d
// Simplify and split:
// q + r = 3
// n + r = 5
// o + p + q = 4
// m + n + p = 7

// Solve:
// min(S)
// where S = m + n + o + p + q + r
// q + r = 3
// n + r = 5
// o + p + q = 4
// m + n + p = 7
// m,n,o,p,q,r >= 0
