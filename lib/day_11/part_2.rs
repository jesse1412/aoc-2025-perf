use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum PathHistory {
    FoundDac(u64),
    FoundFFT(u64),
    Nothing(u64),
    Both(u64),
}

impl PathHistory {
    fn get_inner(&self) -> u64 {
        match self {
            PathHistory::FoundDac(n)
            | PathHistory::FoundFFT(n)
            | PathHistory::Nothing(n)
            | PathHistory::Both(n) => *n,
        }
    }
}

pub fn run(input: &str) -> i64 {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::from_iter([("out", Vec::new())]);
    for line in input.lines() {
        let (from, tos) = line.split_once(':').unwrap();
        for to in tos.split(' ') {
            edges.entry(from).and_modify(|v| v.push(to)).or_default();
        }
    }
    let mut routes_to_exit: HashMap<&str, PathHistory> =
        HashMap::from_iter([("out", PathHistory::Nothing(1))]);
    let mut check_stack: Vec<&str> = vec!["svr"];
    check_stack.append(&mut edges["svr"].clone());
    while let Some(from) = check_stack.pop() {
        let mut miss = false;
        let mut tot_dac_dist = 0;
        let mut tot_fft_dist = 0;
        let mut tot_neither_dist = 0;
        let mut tot_both_dist = 0;
        // println!("{from}, S: {check_stack:?}, {routes_to_exit:?}");

        for to in edges.get(from).unwrap() {
            if let Some(dist) = routes_to_exit.get(to) {
                match dist {
                    PathHistory::FoundFFT(rhs) => tot_fft_dist += rhs,
                    PathHistory::FoundDac(rhs) => tot_dac_dist += rhs,
                    PathHistory::Both(rhs) => tot_both_dist += rhs,
                    PathHistory::Nothing(rhs) => tot_neither_dist += rhs,
                };
            } else {
                if !miss {
                    check_stack.push(from);
                }
                miss = true;
                check_stack.push(to);
            }
        }
        if miss {
            continue;
        }
        let dist = if tot_both_dist > 0 {
            PathHistory::Both(tot_both_dist)
        } else {
            match from {
                "fft" => {
                    if tot_dac_dist > 0 {
                        PathHistory::Both(tot_dac_dist + tot_fft_dist)
                    } else {
                        PathHistory::FoundFFT(tot_fft_dist + tot_neither_dist)
                    }
                }
                "dac" => {
                    if tot_fft_dist > 0 {
                        PathHistory::Both(tot_dac_dist + tot_fft_dist)
                    } else {
                        PathHistory::FoundDac(tot_dac_dist + tot_neither_dist)
                    }
                }
                _ => {
                    if tot_dac_dist > 0 {
                        PathHistory::FoundDac(tot_dac_dist)
                    } else if tot_fft_dist > 0 {
                        PathHistory::FoundFFT(tot_fft_dist)
                    } else {
                        PathHistory::Nothing(tot_neither_dist)
                    }
                }
            }
        };
        routes_to_exit.insert(from, dist);
    }
    routes_to_exit.get("svr").unwrap().get_inner() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(
            run(&"svr: a
a: fft
fft: dac
dac: out"),
            1
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(
            run(&"svr: b c
b: fft
c: fft
fft: dac
dac: out"),
            2
        );
    }

    #[test]
    fn test_c() {
        assert_eq!(
            run(&"svr: fft dac
fft: dac
dac: out"),
            1
        );
    }

    #[test]
    fn test_d() {
        assert_eq!(
            run(&"svr: fft
fft: a b
a: dac
b: dac
dac: out
"),
            2
        );
    }
}
