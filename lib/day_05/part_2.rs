pub fn run(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut start_end_pairs: Vec<(u64, u64)> = Vec::new();
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let (start, end) = line.split_once('-').expect("has dash");
        start_end_pairs.push((start.parse().unwrap(), end.parse().unwrap()));
    }
    start_end_pairs.sort_unstable();
    let mut blended_start_ends = vec![(0, 0)];
    for (start, end) in start_end_pairs {
        if start <= blended_start_ends.last().unwrap().1 {
            blended_start_ends.last_mut().unwrap().1 =
                end.max(blended_start_ends.last_mut().unwrap().1);
        } else {
            blended_start_ends.push((start, end));
        }
    }
    let mut res = 0;
    for (start, end) in blended_start_ends {
        if start == 0 && end == 0 {
            continue;
        } else {
            res += (end - start) + 1;
        }
    }
    res as i64
}
