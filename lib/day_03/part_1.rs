pub fn run(input: &str) -> i64 {
    let mut tot = 0;
    for (i, l) in input.lines().enumerate() {
        let mut l_chars = l.chars().peekable();
        let (mut first_digit, mut second_digit) = ('0', '0');
        while let Some(c) = l_chars.next() {
            if c > first_digit {
                if l_chars.peek().is_some() {
                    first_digit = c;
                    second_digit = '0';
                } else {
                    second_digit = c;
                }
            } else if c > second_digit {
                second_digit = c;
            }
        }
        let num: u32 = [first_digit, second_digit]
            .iter()
            .collect::<String>()
            .parse()
            .expect("valid num");
        // println!("{} - {num}", i + 1);
        tot += num;
    }
    tot as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_143() {
        assert_eq!(run("143"), 43);
    }
    #[test]
    pub fn test_344() {
        assert_eq!(run("344"), 44);
    }
    #[test]
    pub fn test_987654321111111() {
        assert_eq!(run("987654321111111"), 98);
    }
    #[test]
    pub fn test_819() {
        assert_eq!(run("819"), 89);
    }
    #[test]
    pub fn test_234234234234278() {
        assert_eq!(run("234234234234278"), 78);
    }
    #[test]
    pub fn test_818181911112111() {
        assert_eq!(run("818181911112111"), 92);
    }
    #[test]
    pub fn test_811111111111119() {
        assert_eq!(run("811111111111119"), 89);
    }
    #[test]
    pub fn test_a() {
        assert_eq!(run("143"), 43);
    }
    #[test]
    pub fn test_b() {
        assert_eq!(run("144"), 44);
    }
    #[test]
    pub fn test_c() {
        assert_eq!(run("414"), 44);
    }
    #[test]
    pub fn test_d() {
        assert_eq!(run("3454"), 54);
    }
    #[test]
    pub fn test_e() {
        assert_eq!(run("1234554321"), 55);
    }

    #[test]
    pub fn test_4655() {
        assert_eq!(run("4655"), 65)
    }
}
