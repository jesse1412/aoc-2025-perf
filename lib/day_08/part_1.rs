use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
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
    let coords: Vec<(OrdF32, OrdF32, OrdF32)> = input
        .lines()
        .map(|s| {
            let mut split = s.split(",");
            (
                OrdF32(split.next().unwrap().parse().unwrap()),
                OrdF32(split.next().unwrap().parse().unwrap()),
                OrdF32(split.next().unwrap().parse().unwrap()),
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
            let x_dif_sq = (x1.0 - x2.0).powi(2);
            let y_dif_sq = (y1.0 - y2.0).powi(2);
            let z_dif_sq = (z1.0 - z2.0).powi(2);
            let distance = x_dif_sq + y_dif_sq + z_dif_sq;
            closest_ordered.push((Reverse(OrdF32(distance)), (i, j)));
        }
    }
    let mut id_group_map: Vec<Option<usize>> = vec![None; coords.len()];
    let mut group_count = 0;
    let mut connections_left = 10;
    while let Some((_, (i, j))) = closest_ordered.pop() {
        if connections_left == 0 {
            break;
        }
        connections_left -= 1;

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
                id_group_map.iter_mut().for_each(|v| match v {
                    Some(v) if *v == j_group => *v = i_group,
                    _ => (),
                });
            }
        };
    }
    let mut id_counts_in_groups = vec![0; group_count];
    for group in id_group_map.into_iter() {
        if let Some(group) = group {
            id_counts_in_groups[group] += 1;
        } else {
            id_counts_in_groups.push(1);
        }
    }
    id_counts_in_groups.sort_unstable();
    id_counts_in_groups.pop().unwrap()
        * id_counts_in_groups.pop().unwrap()
        * id_counts_in_groups.pop().unwrap()
}
