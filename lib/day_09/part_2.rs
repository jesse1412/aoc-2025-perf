use std::{cmp::Reverse, collections::HashMap};

pub fn run(input: &str) -> i64 {
    let mut compressed_x_to_x: Vec<usize> = Vec::new();
    let mut compressed_y_to_y: Vec<usize> = Vec::new();
    let mut xy_values: Vec<(usize, usize)> = Vec::new();
    for line in input.lines() {
        // println!("{line:?}");
        let split = line.split_once(",").unwrap();
        // println!("{split:?}");
        let (x, y): (usize, usize) = { (split.0.parse().unwrap(), split.1.parse().unwrap()) };
        compressed_x_to_x.push(x);
        compressed_y_to_y.push(y);
        xy_values.push((x, y));
    }
    compressed_x_to_x.sort_unstable();
    compressed_x_to_x.dedup();
    compressed_y_to_y.sort_unstable();
    compressed_y_to_y.dedup();
    let x_to_compressed: HashMap<usize, usize> = compressed_x_to_x
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i))
        .collect();
    let y_to_compressed: HashMap<usize, usize> = compressed_y_to_y
        .iter()
        .enumerate()
        .map(|(i, y)| (*y, i))
        .collect();
    let xy_compressed: Vec<(usize, usize)> = xy_values
        .iter()
        .map(|(x, y)| {
            (
                *x_to_compressed.get(x).unwrap(),
                *y_to_compressed.get(y).unwrap(),
            )
        })
        .collect();
    let mut grid_compressed =
        vec![vec![true; compressed_x_to_x.len() * 2 + 2]; compressed_y_to_y.len() * 2 + 2];
    // println!("{:?}", xy_compressed);
    connect_dots(&xy_compressed, &mut grid_compressed, false);
    // for l in grid_compressed
    //     .iter()
    //     .map(|v| v.iter().map(|b| *b as u8).collect::<Vec<u8>>())
    //     .collect::<Vec<Vec<u8>>>()
    // {
    //     println!("{:?}", l);
    // }
    // Now let's flood fill the compressed grid from the centre.
    // We're making an assumption that the centre falls within the shape and isn't already T.
    let mut to_check = vec![(0, 0)];
    while let Some((x, y)) = to_check.pop() {
        if !grid_compressed[y][x] {
            continue;
        }
        grid_compressed[y][x] = false;
        if x < grid_compressed[0].len() - 1 {
            to_check.push((x + 1, y));
        }
        if x > 0 {
            to_check.push((x - 1, y));
        }
        if y < grid_compressed.len() - 1 {
            to_check.push((x, y + 1));
        }
        if y > 0 {
            to_check.push((x, y - 1));
        }
    }
    connect_dots(&xy_compressed, &mut grid_compressed, true);
    // Now I think it's quite simple, compare every compressed with every other compressed.
    // Minor heuristic-based optimisation, try them in reverse size order.
    let point_to_point_sorted_by_area = {
        let mut point_to_point_sorted_by_area = Vec::new();
        for (i, (x1, y1)) in xy_values.iter().enumerate() {
            for (x2, y2) in xy_values[i + 1..].iter() {
                point_to_point_sorted_by_area.push((
                    (1 + x1.abs_diff(*x2)) as i64 * (1 + y1.abs_diff(*y2)) as i64,
                    ((*x1, *y1), (*x2, *y2)),
                ));
            }
        }
        // let mut xy_compressed = xy_compressed.clone();
        // xy_compressed.sort_unstable_by_key(|(x, y)| *x * *y);
        // xy_compressed
        point_to_point_sorted_by_area.sort_unstable_by_key(|(area, _)| Reverse(*area));
        point_to_point_sorted_by_area
    };
    // println!("");
    // For test input only as one of the assumptions doesn't hold:
    // grid_compressed[1][2] = true;
    // for l in grid_compressed
    //     .iter()
    //     .map(|v| v.iter().map(|b| *b as u8).collect::<Vec<u8>>())
    //     .collect::<Vec<Vec<u8>>>()
    // {
    //     println!("{:?}", l);
    // }
    // println!("ZZZ: {point_to_point_sorted_by_area:?}");
    'outer: for (area, ((x1, y1), (x2, y2))) in point_to_point_sorted_by_area {
        let x1_c = *x_to_compressed.get(&x1).unwrap() * 2 + 1;
        let x2_c = *x_to_compressed.get(&x2).unwrap() * 2 + 1;
        let y1_c = *y_to_compressed.get(&y1).unwrap() * 2 + 1;
        let y2_c = *y_to_compressed.get(&y2).unwrap() * 2 + 1;
        // println!("A {x1_c},{y1_c} {x2_c},{y2_c}     {area}");
        // println!(
        //     "B {},{} {},{}     {area}",
        //     compressed_x_to_x[(x1_c - 1) / 2],
        //     compressed_y_to_y[(y1_c - 1) / 2],
        //     compressed_x_to_x[(x2_c - 1) / 2],
        //     compressed_y_to_y[(y2_c - 1) / 2]
        // );
        let l_x = x1_c.min(x2_c);
        let r_x = x1_c.max(x2_c);
        let l_y = y1_c.min(y2_c);
        let r_y = y1_c.max(y2_c);
        for x in l_x..=r_x {
            // println!("{x}");
            // println!("{y}");
            if !grid_compressed[y1_c][x] || !grid_compressed[y2_c][x] {
                continue 'outer;
            }
            // println!("{x}, {y1_c}");
            // println!("{x}, {y2_c}");
        }
        for y in l_y..=r_y {
            if !grid_compressed[y][x1_c] || !grid_compressed[y][x2_c] {
                continue 'outer;
            }
            // println!("{x1_c}, {y}");
            // println!("{x2_c}, {y}");
        }
        return area;
    }
    unreachable!();
}

fn connect_dots(xy_compressed: &[(usize, usize)], grid_compressed: &mut Vec<Vec<bool>>, b: bool) {
    for ((x1_c, y1_c), (x2_c, y2_c)) in xy_compressed
        .iter()
        .zip(xy_compressed.iter().skip(1).chain(&[xy_compressed[0]]))
    {
        let x1_c = x1_c * 2 + 1;
        let x2_c = x2_c * 2 + 1;
        let y1_c = y1_c * 2 + 1;
        let y2_c = y2_c * 2 + 1;
        let l_x = x1_c.min(x2_c);
        let r_x = x1_c.max(x2_c);
        for x in l_x..=r_x {
            // println!("{x}, {y1_c}");
            // println!("{x}, {y2_c}");
            grid_compressed[y1_c][x] = b;
            grid_compressed[y2_c][x] = b;
        }
        let l_y = y1_c.min(y2_c);
        let r_y = y1_c.max(y2_c);
        for y in l_y..=r_y {
            // println!("{x1_c}, {y}");
            // println!("{x2_c}, {y}");
            grid_compressed[y][x1_c] = b;
            grid_compressed[y][x2_c] = b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        let s = "0,0
4,0
4,2
3,2
3,4
4,4
4,6
0,6
0,4
1,4
1,2
0,2
";
        let res = run(s);
        assert_eq!(res, 15);
    }

    #[test]
    fn test_b() {
        let s = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
        let res = run(s);
        assert_eq!(res, 24);
    }

    #[test]
    fn test_c() {
        let s = "1,1
11,1
11,11
10,11
10,2
1,2
";
        let res = run(s);
        assert_eq!(res, 22);
    }

    #[test]
    fn test_d() {
        let s = "0,0
6,0
6,6
0,6
0,4
4,4
4,2
0,2
0,0
";
        let res = run(s);
        assert_eq!(res, 21);
    }
}
