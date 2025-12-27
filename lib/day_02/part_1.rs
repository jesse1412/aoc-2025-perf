pub fn run(input: &str) -> i64 {
    let mut tot = 0;
    for range_str in input.split(',') {
        let (start, end) = range_str.split_once('-').expect("exists");
        tot += calculate_sum_of_repeats(start, end);
    }
    tot
}

fn calculate_sum_of_repeats(start_str: &str, end_str: &str) -> i64 {
    let start_val: i64 = start_str.parse().unwrap();
    let end_val: i64 = end_str.parse().unwrap();
    let mut repeat_nums_sum = 0;

    for len in start_str.len()..=end_str.len() {
        if len % 2 != 0 {
            continue;
        }

        let half_len = (len / 2) as u32;
        // E.g to repeat "65",
        // we need to shift it left 2 places, then add 65.
        // That would mean multiplying by 100, and adding 65.
        // Or, multiplying by 101. This var is 101 in that example.
        let multiplier_to_make_repeating_number = 10_i64.pow(half_len) + 1;

        // For 6531, at root len 2
        // we're counting 10 10,11 11,12 12,13 13...63 63,
        // so all the doubles between 10 and 99 that fit.
        let min_root_for_len = 10_i64.pow(half_len - 1);
        let max_root_for_len = 10_i64.pow(half_len) - 1;

        // What is our first root value that's greater than our min?
        // Do the math for our example (6531):
        // repeat_val * multiplier_to_make_repeating_number >= start_val.
        // Rearange:
        // repeat_val > start_val / multiplier_to_make_repeating_number.
        // repeat_val > 6531 / 101
        // repeat_val > 64.66
        // repeat_val = 65.
        let first_root = ((start_val + multiplier_to_make_repeating_number - 1)
            / multiplier_to_make_repeating_number)
            .max(min_root_for_len);

        let last_root = (end_val / multiplier_to_make_repeating_number).min(max_root_for_len);

        if first_root <= last_root {
            // Now just a bit of math to add the repeated number.
            // n(n+1) / 2
            let nodes_to_add = last_root * (last_root + 1) / 2; // range 0..last_root.
            let nodes_to_sub = (first_root - 1) * (first_root) / 2; // range 0..first_root - 1.
            let val_to_add = multiplier_to_make_repeating_number * (nodes_to_add - nodes_to_sub); // range first_root..last_root.
            repeat_nums_sum += val_to_add;
        }
    }
    repeat_nums_sum
}
