pub fn run(input: &str) -> i64 {
    const NUM_INCLUDED: usize = 12;
    let mut tot = 0;
    for l in input.lines() {
        let mut l_chars = l.chars().enumerate().peekable();
        let mut digits = ['0'; NUM_INCLUDED];
        for (c_index, c) in l_chars {
            for digit_index in 0..digits.len() {
                let digit = digits[digit_index];
                if c > digit {
                    let remaining_chars = l.len() - (c_index + 1);
                    let remaining_digits = digits.len() - (digit_index + 1);
                    // If there's a number left for every following digit.
                    if remaining_digits <= remaining_chars {
                        digits[digit_index] = c;
                        // Zero out every following digit.
                        for x in digit_index + 1..digits.len() {
                            digits[x] = '0';
                        }
                        break;
                    }
                }
            }
        }
        let num: u64 = digits
            .iter()
            .collect::<String>()
            .parse()
            .expect("valid num");
        tot += num;
    }
    tot as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_987654321111111() {
        assert_eq!(run("987654321111111"), 987654321111);
    }
}
