use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> i64 {
    let res = {
        let mut res = 0;
        for line in input.lines() {
            res += handle_line(line);
            // println!("res change {res}");
        }
        res
    } as i64;
    // Logic: Convert buttons into bitmasks.
    // Find bitmasks which XOR to the target.
    // 0101 target
    // 1001
    // 1100
    // 0111
    // 1111
    // 0011
    // Logic: Start at the target, apply a button's mask, if at 0, win.
    // If not at 0, cache state and try next button's mask.
    // If not at 0 after all buttons, try again starting at each cached state.
    // Can maybe reduce the number of comparisons done for this (only loop through new cache entries).
    // Longest light mask is 10 digits, so a 2^10 sized vec can be used for cache instead of hm.
    res
}

fn handle_line(line: &str) -> u64 {
    let line_split_vec: Vec<&str> = line.split(' ').collect();

    let indicator_lights = line_split_vec.first().unwrap();
    let indicator_lights = &indicator_lights
        .strip_prefix('[')
        .unwrap()
        .strip_suffix(']')
        .unwrap();
    let indicator_lights_mask = {
        let mut indicator_lights_mask = 0;
        for (i, c) in indicator_lights.chars().enumerate() {
            match c {
                '#' => indicator_lights_mask += 2_u64.pow(i as u32),
                _ => (),
            }
        }
        indicator_lights_mask
    };

    if indicator_lights_mask == 0 {
        return 0;
    }

    // println!("lights: {indicator_lights_mask:#b}");

    let buttons = &line_split_vec[1..line_split_vec.len() - 1];
    // println!("buttons {buttons:?}");
    let button_bitmasks: HashSet<u64> = buttons
        .into_iter()
        .map(|s| {
            // println!("{s}");
            s.strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(',')
                .fold(0, |acc, button| {
                    // println!("{acc}: {:b}", acc + 2_u64.pow(button.parse().unwrap()));
                    acc + 2_u64.pow(button.parse().unwrap())
                })
        })
        .collect();

    if button_bitmasks.contains(&indicator_lights_mask) {
        return 1;
    }

    // for button_bitmask in button_bitmasks.iter() {
    //     println!("button: {button_bitmask:#b}");
    // }

    let mut next_mask_moves_from_finish: HashMap<u64, usize> =
        HashMap::from_iter([(indicator_lights_mask, 0)]);
    let mut mask_moves_from_finish: HashMap<u64, usize> = next_mask_moves_from_finish.clone();

    loop {
        for (mask, moves_from_finish) in mask_moves_from_finish.into_iter() {
            for button_mask in button_bitmasks.iter().cloned() {
                // println!("{next_mask_moves_from_finish:?}");
                let new_mask = mask ^ button_mask;
                // println!(
                //     "mask: {mask:#b} button: {button_mask:#b} new: {new_mask:#b} away (+1): {}",
                //     moves_from_finish + 1
                // );
                // println!("{new_mask:b}: {moves_from_finish}");
                if new_mask == 0 {
                    // next_mask_moves_from_finish.insert(new_mask, moves_from_finish + 1);
                    // for (button_bitmask, away) in next_mask_moves_from_finish.into_iter() {
                    //     println!("mask: {button_bitmask:#b}, away: {away}");
                    // }
                    return (moves_from_finish + 1) as u64;
                }
                if next_mask_moves_from_finish.contains_key(&new_mask) {
                    continue;
                }
                next_mask_moves_from_finish.insert(new_mask, moves_from_finish + 1);
            }
        }
        mask_moves_from_finish = next_mask_moves_from_finish.clone();
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(run(&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"), 2);
    }

    #[test]
    fn test_b() {
        assert_eq!(
            run(&"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            3
        );
    }

    #[test]
    fn test_c() {
        assert_eq!(
            run(&"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
            2
        );
    }

    #[test]
    fn test_d() {
        assert_eq!(run(&"[#] (0) {1}"), 1);
    }

    #[test]
    fn test_e() {
        assert_eq!(run(&"[.] (0) {1}"), 0);
    }
    #[test]
    fn test_f() {
        assert_eq!(run(&"[####] (0,1) (1) (2,1) (3) {1}"), 4);
    }
}
