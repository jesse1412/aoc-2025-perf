pub fn run(input: &str) -> i64 {
    let mut max_y = 0;
    let mut coords_sorted_manhatten: Vec<(u32, u32)> = input
        .lines()
        .map(|s| {
            let split = s.split_once(",").unwrap();
            let y: u32 = split.1.parse().unwrap();
            max_y = y.max(max_y);
            (split.0.parse().unwrap(), y)
        })
        .collect();
    coords_sorted_manhatten.sort_unstable_by_key(|(x, y)| x + y);

    // Now we just need to get the lowest and highest bunches.
    // Start with highest for x + y.
    let mut largest_area = calc_highest_area(&coords_sorted_manhatten);

    // Now do it all again on the transposed grid.
    // The above only works in the nw -> se direction.
    // So let's do sw -> ne by transposing.
    let mut coords_sorted_manhatten_transposed: Vec<(u32, u32)> = coords_sorted_manhatten.clone();
    coords_sorted_manhatten_transposed.sort_unstable_by_key(|(x, y)| x + (max_y - 1) - y);
    largest_area = largest_area.max(calc_highest_area(&coords_sorted_manhatten_transposed));

    largest_area
}

fn calc_highest_area(coords_sorted_manhatten: &Vec<(u32, u32)>) -> i64 {
    let highest_manhatten_distance =
        coords_sorted_manhatten.last().unwrap().0 + coords_sorted_manhatten.last().unwrap().1;
    let mut all_highest_manhatten = Vec::new();
    let mut coords_sorted_manhatten_rev_iter = coords_sorted_manhatten.iter().rev();
    while let Some((x, y)) = coords_sorted_manhatten_rev_iter.next()
        && x + y == highest_manhatten_distance
    {
        all_highest_manhatten.push((x, y));
    }
    // Now lowest
    let lowest_manhatten_distance =
        coords_sorted_manhatten.first().unwrap().0 + coords_sorted_manhatten.first().unwrap().1;
    let mut all_lowest_manhatten = Vec::new();
    let mut coords_sorted_manhatten_iter = coords_sorted_manhatten.iter();
    while let Some((x, y)) = coords_sorted_manhatten_iter.next()
        && x + y == lowest_manhatten_distance
    {
        all_lowest_manhatten.push((x, y));
    }
    let mut largest_area = 0;
    // println!("{lowest_xpy_collection:?}");
    // println!("{all_top_sorted:?}");
    // Now compare them all and get the biggest area.
    for (x1, y1) in all_highest_manhatten {
        for (x2, y2) in all_lowest_manhatten.iter() {
            largest_area =
                largest_area.max((x1.abs_diff(**x2) + 1) as i64 * (y1.abs_diff(**y2) + 1) as i64);
        }
    }
    largest_area
}
