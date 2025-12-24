pub fn run(input: &str) -> i64 {
    let mut out = 0;
    let mut lines = input.lines().peekable();
    let mut cells_per_present = Vec::new();
    while lines.peek().unwrap().ends_with(':') {
        lines.next();
        let mut present_cell_count: usize =
            lines.next().unwrap().chars().fold(0, count_present_cells);
        while let Some(line) = lines.next()
            && !line.is_empty()
        {
            present_cell_count += line.chars().fold(0, count_present_cells);
        }
        cells_per_present.push(present_cell_count);
    }
    for line in lines {
        let (dims, counts) = line.split_once(':').unwrap();
        let (x_dim, y_dim) = dims.split_once('x').unwrap();
        let x_dim = x_dim.parse::<usize>().unwrap();
        let y_dim = y_dim.parse::<usize>().unwrap();
        let area = x_dim * y_dim;
        let cells_used_min: usize = counts
            .split(' ')
            .skip(1)
            .enumerate()
            .map(|(present_id, count)| {
                cells_per_present[present_id] * count.parse::<usize>().unwrap()
            })
            .sum();
        if cells_used_min <= area {
            out += 1;
        }
    }
    out
}

fn count_present_cells(acc: usize, v: char) -> usize {
    acc + match v {
        '#' => 1,
        _ => 0,
    }
}
