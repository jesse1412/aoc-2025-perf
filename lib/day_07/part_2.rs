pub fn run(input: &str) -> i64 {
    let mut char_grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let y_max = char_grid.len();
    let x_max = char_grid[0].len();
    let mut path_counts = vec![vec![0_u64; x_max]; y_max];
    for y in 1..y_max {
        for x in 0..x_max {
            let curr = char_grid[y][x];
            let above = char_grid[y - 1][x];
            match above {
                'S' => {
                    path_counts[y][x] = 1;
                    char_grid[y][x] = '|';
                }
                '|' => match curr {
                    '^' => {
                        if x > 0 {
                            char_grid[y][x - 1] = '|';
                            path_counts[y][x - 1] += path_counts[y - 1][x]
                        }
                        if x < x_max {
                            char_grid[y][x + 1] = '|';
                            path_counts[y][x + 1] += path_counts[y - 1][x]
                        }
                    }
                    _ => {
                        char_grid[y][x] = '|';
                        path_counts[y][x] += path_counts[y - 1][x]
                    }
                },
                '^' => (),
                _ => (),
            }
        }
    }

    path_counts.pop().unwrap().into_iter().sum::<u64>() as i64
}
