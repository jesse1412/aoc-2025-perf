enum Operator {
    Add,
    Mul,
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => unreachable!(),
        }
    }
}

pub fn run(input: &str) -> i64 {
    let mut parsed_lines: Vec<Vec<&str>> = Vec::new();
    for line in input.lines() {
        parsed_lines.push(line.split_whitespace().collect());
    }
    let operators = parsed_lines.pop().unwrap();
    let line_count = parsed_lines.len();
    let mut overall_total = 0;
    for x in 0..operators.len() {
        let operater = Operator::from(operators[x]);
        let mut column_total = match operater {
            Operator::Add => 0,
            Operator::Mul => 1,
        };
        for y in 0..line_count {
            let n: u64 = parsed_lines[y][x].parse().unwrap();
            match operater {
                Operator::Add => column_total += n,
                Operator::Mul => column_total *= n,
            }
        }
        overall_total += column_total
    }
    overall_total as i64
}
