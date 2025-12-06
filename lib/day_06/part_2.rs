enum Operator {
    Add,
    Mul,
}

pub fn run(input: &str) -> i64 {
    let mut char_grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut operators: Vec<Operator> = char_grid
        .pop()
        .unwrap()
        .iter()
        .filter_map(|c| match c {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Mul),
            _ => None,
        })
        .collect();
    let mut curr_num_chars: Vec<char> = Vec::new();
    let mut operator = operators.pop().unwrap();
    let mut computed_nums = vec![match operator {
        Operator::Add => 0,
        Operator::Mul => 1,
    }];
    for x in (0..char_grid.first().unwrap().len()).rev() {
        curr_num_chars.clear();
        for y in 0..char_grid.len() {
            let c = char_grid[y][x];
            if !c.is_whitespace() {
                curr_num_chars.push(c);
            }
        }
        if curr_num_chars.is_empty() {
            if let Some(op) = operators.pop() {
                operator = op;
                computed_nums.push(match operator {
                    Operator::Add => 0,
                    Operator::Mul => 1,
                });
            }
        } else {
            println!("{curr_num_chars:?}");
            let num: u64 = curr_num_chars.iter().collect::<String>().parse().unwrap();
            match operator {
                Operator::Add => *computed_nums.last_mut().unwrap() += num,
                Operator::Mul => *computed_nums.last_mut().unwrap() *= num,
            }
        }
    }
    println!("{computed_nums:?}");
    computed_nums.into_iter().sum::<u64>() as i64
}
