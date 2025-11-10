use crate::day_02::is_valid_check;

pub fn run(input: &str) -> i64 {
    let mut tot = 0;

    for line in input.lines() {
        let nums: Vec<u8> = line.split(" ").map(|s| s.parse::<u8>().unwrap()).collect();
        let (mut add_amount, failed_pos) = is_valid_check(nums.iter());
        if add_amount == 0 {
            let (reattempt_amt_3, _) = is_valid_check(
                nums[0..failed_pos - 1]
                    .iter()
                    .chain(nums[failed_pos..].iter()),
            );
            let (reattempt_amt, _) = is_valid_check(
                nums[0..failed_pos]
                    .iter()
                    .chain(nums[failed_pos + 1..].iter()),
            );
            let (reattempt_amt_2, _) = is_valid_check(
                nums[0..failed_pos + 1]
                    .iter()
                    .chain(nums[failed_pos + 2..].iter()),
            );
            if reattempt_amt > 0 || reattempt_amt_2 > 0 || reattempt_amt_3 > 0 {
                add_amount = 1;
            }
        }
        tot += add_amount;
    }
    tot as i64
}
