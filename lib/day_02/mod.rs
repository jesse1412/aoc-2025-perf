mod part_1;
mod part_2;

pub use part_1::run as p2;
pub use part_2::run as p1;

fn is_valid_check<'a>(mut nums: impl Iterator<Item = &'a u8>) -> (u16, usize) {
    let (mut prev, mut curr) = (nums.next().unwrap(), nums.next().unwrap());
    let direction: Direction = if let Some(dir) = Direction::from_u8s(*curr, *prev) {
        dir
    } else {
        return (0, 1);
    };
    if do_comp(prev, curr, &direction) == 0 {
        return (0, 1);
    }
    for (i, n) in nums.enumerate() {
        prev = curr;
        curr = n;
        if do_comp(prev, curr, &direction) == 0 {
            return (0, i + 1);
        }
    }
    (1, 0)
}

enum Direction {
    Increasing,
    Decreasing,
}

impl Direction {
    fn from_u8s(curr: u8, prev: u8) -> Option<Self> {
        match prev.cmp(&curr) {
            std::cmp::Ordering::Greater => Some(Self::Decreasing),
            std::cmp::Ordering::Less => Some(Self::Increasing),
            std::cmp::Ordering::Equal => None,
        }
    }
}

#[inline(always)]
fn do_comp<'a>(prev: &'a u8, curr: &'a u8, direction: &Direction) -> u16 {
    match *direction {
        Direction::Decreasing => {
            if curr >= prev || curr.abs_diff(*prev) > 3 {
                return 0;
            }
        }
        Direction::Increasing => {
            if curr <= prev || curr.abs_diff(*prev) > 3 {
                return 0;
            }
        }
    }
    1
}
