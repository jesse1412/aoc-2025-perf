use super::get_digits;

const STORED_VALUE_LOOKUP_MAP: u32 = 0b11111111111111111000000000000000;
const STORED_RHS_COUNT_LOOKUP_MAP: u32 = 0b111111111111111;

pub fn run(input: &str) -> i64 {
    // TODO: Store LHS and RHS counts in the hashmap?
    let input = input.as_bytes();

    let end_line = input.len() / 14;

    let mut left_list: Vec<u32> = Vec::with_capacity(1000);
    // Position stores the first 10 bits of our number.
    // First 17 bits of the actual value stores the number in our hashmap.
    // Last 15 bits store the count.
    let mut counts_hashmap: [u32; 2usize.pow(12)] = [0; 2usize.pow(12)];

    // println!("const: {STORED_VALUE_LOOKUP_MAP:b}");
    get_counts(input, end_line, &mut left_list, &mut counts_hashmap);
    let mut tot = 0;
    compute_total_distances(left_list, counts_hashmap, &mut tot);
    tot as i64
}

fn get_counts(
    input: &[u8],
    end_line: usize,
    left_list: &mut Vec<u32>,
    counts_hashmap: &mut [u32; 4096],
) {
    for curr_line in 0..end_line {
        unsafe {
            let (lhs_number, rhs_number) = get_digits(input, curr_line);
            // println!("LHS/RHS {lhs_number}, {rhs_number}");
            // println!("RHSb {rhs_number:b}");
            // println!("hash {:b}", rhs_number >> 7);
            left_list.push(lhs_number);
            let mut hash = (rhs_number >> 5) as usize;
            loop {
                let v = counts_hashmap.get_unchecked(hash);
                let val_in_slot = v & STORED_VALUE_LOOKUP_MAP;
                let rhs_number_shifted_15_l = rhs_number << 15;
                // println!("val_in_slot {val_in_slot:b}");
                if val_in_slot == 0 || val_in_slot == rhs_number_shifted_15_l {
                    // println!("PRE counts_hashmap[hash]  {}", counts_hashmap[hash]);
                    // println!("PRE counts_hashmap[hash]b {:b}", counts_hashmap[hash]);
                    counts_hashmap[hash] = (v | rhs_number_shifted_15_l) + 1;
                    // println!("POST counts_hashmap[hash] {}", counts_hashmap[hash]);
                    // println!("POST counts_hashmap[hsh]b {:b}", counts_hashmap[hash]);
                    // println!(
                    //     "New count: {}",
                    //     counts_hashmap[hash] & STORED_COUNT_LOOKUP_MAP
                    // );
                    break;
                }
                hash += 1;
            }
        }
    }
}

fn compute_total_distances(left_list: Vec<u32>, counts_hashmap: [u32; 4096], tot: &mut u64) {
    // let numbers = left_list.len();
    // let mut loops_total = 0;
    // let mut min_hash = usize::MAX;
    // let mut max_hash = 0;
    for lhs_number in left_list {
        let mut hash = (lhs_number >> 5) as usize;
        // min_hash = min_hash.min(hash);
        // max_hash = max_hash.max(hash);
        loop {
            let v = counts_hashmap[hash];
            // loops_total += 1;
            let val_in_slot = v & STORED_VALUE_LOOKUP_MAP;
            if val_in_slot == 0 {
                break;
            }
            if val_in_slot == lhs_number << 15 {
                let count_in_slot = v & STORED_RHS_COUNT_LOOKUP_MAP;
                // println!("Num {lhs_number} found {count_in_slot} time");
                *tot += (lhs_number * count_in_slot) as u64;
                break;
            }
            hash += 1;
            // println!("val_in_slot {val_in_slot:b}");
        }
        // println!("#{numbers} - tot {loops_total}");
        // let r_count = *counts_hashmap.get(&hash).unwrap_or(&0) as u64;
        // tot += l as u64 * r_count;
    }
    // println!("min hash: {min_hash}, max hash: {max_hash}")
}
