pub fn run(input: &str) -> i64 {
    let mut char_grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let y_max = char_grid.len();
    let x_max = char_grid[0].len();
    let mut split_count = 0;
    for y in 1..y_max {
        for x in 0..x_max {
            let curr = char_grid[y][x];
            let above = char_grid[y - 1][x];
            match above {
                'S' => {
                    char_grid[y][x] = '|';
                }
                '|' => match curr {
                    '^' => {
                        split_count += 1;
                        if x > 0 {
                            char_grid[y][x - 1] = '|';
                        }
                        if x < x_max {
                            char_grid[y][x + 1] = '|';
                        }
                    }
                    _ => char_grid[y][x] = '|',
                },
                '^' => (),
                _ => (),
            }
        }
    }
    for row in char_grid {
        println!("{:}", row.into_iter().collect::<String>());
    }
    split_count
}
