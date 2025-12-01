pub fn run(input: &str) -> i64 {
    let mut tot = 50;
    let mut zeros = 0;
    for l in input.lines() {
        let mul = match &l[0..1] {
            "R" => 1,
            "L" => -1,
            _ => unreachable!(),
        };
        let number: i16 = l[1..].parse().expect("should be a number");
        tot += mul * number;
        if tot % 100 == 0 {
            zeros += 1;
        }
    }
    zeros as i64
}
