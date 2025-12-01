pub fn run(input: &str) -> i64 {
    let mut tot = 50_i16;
    let mut passes = 0;
    for l in input.lines() {
        let starting_tot_sign = tot.signum();
        let mul = match &l[0..1] {
            "R" => 1,
            "L" => -1,
            _ => unreachable!(),
        };
        let number: i16 = l[1..].parse().expect("should be a number");
        let diff = number * mul;

        tot += diff;
        let pass_flag = match (starting_tot_sign, tot.signum()) {
            (-1, 1) | (1, -1) => 1,
            (_, 0) => 1,
            _ => 0,
        };
        passes += (tot / 100).abs() + pass_flag as i16;
        tot %= 100;
    }
    passes as i64
}

/*
Need to check for going past 100, 0, or -100
old num 55 -> new num 205. Gone past 100 and 200, via 205 / 100.
old num 55 -> new num -101. Gone past -100 and 0, via -101 / 100 (plus 1 for passing 0).
*/
