use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fmt,
};

#[derive(PartialEq, Clone, Copy)]
pub struct OrdF32(pub f32);

impl Eq for OrdF32 {}

impl fmt::Debug for OrdF32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just show the inner float value
        write!(f, "{}", self.0)
    }
}

impl Ord for OrdF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for OrdF32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(input: &str) -> i64 {
    let coords: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|s| {
            let mut split = s.split(",");
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut closest_ordered: BinaryHeap<(Reverse<OrdF32>, (usize, usize))> =
        BinaryHeap::with_capacity(coords.len() * coords.len());
    for (i, (x1, y1, z1)) in coords.iter().enumerate() {
        for (j, (x2, y2, z2)) in coords.iter().enumerate().skip(i + 1) {
            if i == j {
                continue;
            }
            let x_dif_sq = (x1 - x2).pow(2);
            let y_dif_sq = (y1 - y2).pow(2);
            let z_dif_sq = (z1 - z2).pow(2);
            let distance = x_dif_sq + y_dif_sq + z_dif_sq;
            closest_ordered.push((Reverse(OrdF32(distance as f32)), (i, j)));
        }
    }
    let mut id_group_map: Vec<Option<usize>> = vec![None; coords.len()];
    let mut group_count = 0;
    while let Some((_, (i, j))) = closest_ordered.pop() {
        match (id_group_map[i], id_group_map[j]) {
            (Some(_), None) => {
                id_group_map[j] = id_group_map[i];
            }
            (None, Some(_)) => {
                id_group_map[i] = id_group_map[j];
            }
            (None, None) => {
                id_group_map[i] = Some(group_count);
                id_group_map[j] = Some(group_count);
                group_count += 1;
            }
            (Some(i_group), Some(j_group)) => {
                for v in id_group_map.iter_mut() {
                    match v {
                        Some(v) if *v == j_group => *v = i_group,
                        _ => (),
                    }
                }
            }
        };
        // I honestly just cba to code this up efficiently.
        if HashSet::<&Option<usize>>::from_iter(id_group_map.iter()).len() == 1 {
            return coords[i].0 * coords[j].0;
        }
    }
    unreachable!();
}
