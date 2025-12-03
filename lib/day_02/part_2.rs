pub fn run(input: &str) -> i64 {
    let mut tot = 0;
    for range_str in input.split(',') {
        let (start, end) = range_str.split_once('-').expect("exists");
        let start: i64 = start.parse().expect("valid number");
        let end: i64 = end.parse().expect("valid number");
        for num in start..=end {
            let num_char_bytes = num.to_string().into_bytes();
            if is_valid(&num_char_bytes, &num_char_bytes[0..1]) {
                tot += num;
            }
        }
    }
    tot
}

fn is_valid(num: &[u8], repeated_num: &[u8]) -> bool {
    if repeated_num.len() == num.len() {
        // Stupid edge case for double digits.
        return false;
    }
    for chunk in num.chunks(repeated_num.len()) {
        if chunk != repeated_num {
            if repeated_num.len() > num.len() / 2 {
                return false;
            } else {
                return is_valid(num, &num[0..=repeated_num.len()]);
            }
        }
    }
    true
}
