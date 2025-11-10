use crate::day_02::is_valid_check;

pub fn run(input: &str) -> i64 {
    let mut tot = 0;

    for line in input.lines() {
        let nums: Vec<u8> = line.split(" ").map(|s| s.parse::<u8>().unwrap()).collect();
        let (add_amount, _) = is_valid_check(nums.iter());
        tot += add_amount;
    }
    tot as i64
}
