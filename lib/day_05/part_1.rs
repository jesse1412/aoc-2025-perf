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
    let mut res: u64 = 0;
    // println!("{blended_start_ends:#?}");
    for line in lines {
        let n: u64 = line.parse().unwrap();
        match blended_start_ends.binary_search_by_key(&n, |t| t.0) {
            Ok(_) => {
                res += 1;
                // println!("ok {n}");
            }
            Err(i) => {
                if let Some((start, end)) = blended_start_ends.get(i.saturating_sub(1))
                    && n >= *start
                    && n <= *end
                {
                    res += 1;
                    // println!("ok(err) {n}");
                }
            }
        }
    }
    res as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let res = run("1-5
5-10

4
5
6");
        assert_eq!(res, 3);
    }

    #[test]
    fn test_2() {
        let res = run("5-6
5-7
5-8

4
5
6
7
8
9");
        assert_eq!(res, 4);
    }
    #[test]
    fn test_3() {
        let res = run("8-9
5-8

4
5
6
7
8
9");
        assert_eq!(res, 5);
    }

    #[test]
    fn test_4() {
        let res = run("8-8

4
5
6
7
8
9");
        assert_eq!(res, 1);
    }

    #[test]
    fn test_5() {
        let res = run("5-7
8-10
6-8

4
5
6
7
8
9
10
11");
        assert_eq!(res, 6);
    }

    #[test]
    fn test_6() {
        let res = run("5-7
7-8

4
5
6
7
8
9
10
11");
        assert_eq!(res, 4);
    }

    //     #[test]
    //     fn test_7() {
    //         let res = run("5-7
    // 7-8

    // 0
    // 5
    // 6
    // 7
    // 8
    // 9
    // 10
    // 11");
    //         assert_eq!(res, 4);
    //     }

    #[test]
    fn test_8() {
        let res = run("2-5
3-4

1
2
3
4
5
6");
        assert_eq!(res, 4);
    }
}
