pub fn run(input: &str) -> i64 {
    let mut tot = 0;
    for range_str in input.split(',') {
        let (start, end) = range_str.split_once('-').expect("exists");
        let start: i64 = start.parse().expect("valid number");
        let end: i64 = end.parse().expect("valid number");
        for num in start..=end {
            let num_str = num.to_string();
            if is_valid(&num_str) {
                tot += num;
            }
        }
    }
    tot
}

fn is_valid(num: &str) -> bool {
    if num.len() % 2 == 1 {
        return false;
    }
    let (l, r) = num.split_at(num.len() / 2);
    l == r
}
