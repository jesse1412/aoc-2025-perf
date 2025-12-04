pub fn run(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let y_limit = grid[0].len() as isize;
    let x_limit = grid.len() as isize;
    let mut adjacent_counts: Vec<Vec<u16>> = vec![vec![0; y_limit as usize]; x_limit as usize];
    let mut roll_positions = Vec::new();
    for (y, row) in grid.into_iter().enumerate() {
        for (x, c) in row.into_iter().enumerate() {
            if c == '@' {
                roll_positions.push((y, x));
                add_in_9x9(y_limit, x_limit, &mut adjacent_counts, y, x, 1);
            };
        }
    }
    let mut tot = 0;
    for (y, x) in roll_positions {
        if adjacent_counts[y][x] < 4 {
            tot += 1;
        }
    }
    tot
}

#[inline(always)]
fn add_in_9x9(
    y_limit: isize,
    x_limit: isize,
    adjacent_counts: &mut [Vec<u16>],
    y: usize,
    x: usize,
    add_num: u16,
) {
    for y_offset in -1..2 {
        for x_offset in -1..2 {
            if x_offset == 0 && y_offset == 0 {
                continue;
            }
            let y = match (y as isize) + y_offset {
                actual_y if actual_y < y_limit && actual_y >= 0 => actual_y,
                _ => continue,
            };
            let x = match (x as isize) + x_offset {
                actual_x if actual_x < x_limit && actual_x >= 0 => actual_x,
                _ => continue,
            };
            adjacent_counts[y as usize][x as usize] += add_num;
        }
    }
}
