use std::collections::HashMap;

pub fn run(input: &str) -> i64 {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (from, tos) = line.split_once(':').unwrap();
        for to in tos.split(' ') {
            edges.entry(from).and_modify(|v| v.push(to)).or_default();
        }
    }
    let mut routes_to_exit: HashMap<&str, usize> = HashMap::from_iter([("out", 1)]);
    let mut check_stack: Vec<&str> = vec!["you"];
    check_stack.append(&mut edges["you"].clone());
    while let Some(from) = check_stack.pop() {
        // println!("stack {from}: {check_stack:?}");

        let mut tot_dist = 0;
        let mut miss = false;
        for to in edges.get(from).unwrap() {
            if let Some(dist) = routes_to_exit.get(to) {
                tot_dist += dist
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
        routes_to_exit.insert(from, tot_dist);
    }
    // println!("{routes_to_exit:?}");
    *routes_to_exit.get("you").unwrap() as i64
}
