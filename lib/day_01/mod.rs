mod part_1;
mod part_2;

pub use part_1::run as p2;
pub use part_2::run as p1;

const ASCII_OFFSET: u32 = (b'0' as u32)
    + (b'0' as u32 * 10)
    + (b'0' as u32 * 100)
    + (b'0' as u32 * 1000)
    + (b'0' as u32 * 10000);

#[inline(always)]
unsafe fn get_digits(input: &[u8], curr_line: usize) -> (u32, u32) {
    let lhs_number_start_pos = curr_line * 14;
    let rhs_number_start_pos = curr_line * 14 + 8;
    unsafe {
        let lhs_digit_0 = *input.get_unchecked(lhs_number_start_pos) as u32;
        let lhs_digit_1 = *input.get_unchecked(lhs_number_start_pos + 1) as u32;
        let lhs_digit_2 = *input.get_unchecked(lhs_number_start_pos + 2) as u32;
        let lhs_digit_3 = *input.get_unchecked(lhs_number_start_pos + 3) as u32;
        let lhs_digit_4 = *input.get_unchecked(lhs_number_start_pos + 4) as u32;

        let rhs_digit_0 = *input.get_unchecked(rhs_number_start_pos) as u32;
        let rhs_digit_1 = *input.get_unchecked(rhs_number_start_pos + 1) as u32;
        let rhs_digit_2 = *input.get_unchecked(rhs_number_start_pos + 2) as u32;
        let rhs_digit_3 = *input.get_unchecked(rhs_number_start_pos + 3) as u32;
        let rhs_digit_4 = *input.get_unchecked(rhs_number_start_pos + 4) as u32;

        let lhs_number = lhs_digit_0 * 10000
            + lhs_digit_1 * 1000
            + lhs_digit_2 * 100
            + lhs_digit_3 * 10
            + lhs_digit_4
            - ASCII_OFFSET;
        let rhs_number = rhs_digit_0 * 10000
            + rhs_digit_1 * 1000
            + rhs_digit_2 * 100
            + rhs_digit_3 * 10
            + rhs_digit_4
            - ASCII_OFFSET;
        (lhs_number, rhs_number)
    }
}
