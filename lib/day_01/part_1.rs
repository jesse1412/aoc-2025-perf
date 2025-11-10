use super::get_digits;
use std::mem::MaybeUninit;

pub fn run(input: &str) -> i64 {
    let input = input.as_bytes();

    let end_line = input.len() / 14;
    let mut left_list: [MaybeUninit<u32>; 1000] = [MaybeUninit::uninit(); 1000];
    let mut right_list: [MaybeUninit<u32>; 1000] = [MaybeUninit::uninit(); 1000];
    let mut i = 0;
    for curr_line in 0..end_line {
        unsafe {
            let (lhs_number, rhs_number) = get_digits(input, curr_line);
            left_list[i] = MaybeUninit::new(lhs_number);
            right_list[i] = MaybeUninit::new(rhs_number);
            i += 1;
        }
    }
    unsafe {
        radix_sort_u17(&mut left_list);
        radix_sort_u17(&mut right_list);

        let res = left_list
            .into_iter()
            .zip(right_list)
            .fold(0, |tot: u32, (l, r)| {
                tot.unchecked_add(l.assume_init().abs_diff(r.assume_init()))
            });
        res as i64
    }
}

unsafe fn radix_sort_u17(arr: &mut [MaybeUninit<u32>; 1000]) {
    let mut cnt_lo: [u16; 256] = [0; 256];
    let mut cnt_hi: [u16; 512] = [0; 512];
    unsafe {
        for x in arr.iter() {
            *cnt_lo.get_unchecked_mut((x.assume_init() & 255) as usize) += 1;
            *cnt_hi.get_unchecked_mut((x.assume_init() >> 8) as usize) += 1;
        }

        for i in 0..255 {
            let idx = i + 1;
            cnt_lo[idx] += cnt_lo[i];
            cnt_hi[idx] += cnt_hi[i];
        }
        for i in 255..511 {
            cnt_hi[i + 1] += cnt_hi[i];
        }

        let mut buf = [0; 1000];

        for x in arr.iter().rev() {
            let ptr = cnt_lo.get_unchecked_mut((x.assume_init() & 255) as usize);
            *ptr -= 1;
            *buf.get_unchecked_mut(*ptr as usize) = x.assume_init();
        }

        for x in buf.iter().rev() {
            let ptr = cnt_hi.get_unchecked_mut((*x >> 8) as usize);
            *ptr -= 1;
            *arr.get_unchecked_mut(*ptr as usize) = MaybeUninit::new(*x);
        }
    }
}
