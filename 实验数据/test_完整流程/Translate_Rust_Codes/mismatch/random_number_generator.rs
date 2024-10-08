use std::mem;

type ULL = u64;
const N: usize = mem::size_of::<ULL>() * 8;
const B: fn(usize) -> ULL = |x| 1 << x;

fn evolve(mut state: ULL, rule: ULL) {
    for _p in 0..10 {
        let mut b: u64 = 0;
        for q in (0..8).rev() {
            let mut st = state;
            b |= (st & 1) << q;

            state = 0;
            for i in 0..N {
                // Corrected: Ensure the bitwise shifts are correctly calculated
                let shift_left = (N - 1 - i) % N;
                let shift_right = (i + 1) % N;
                
                let shifted_left = st << shift_left;
                let shifted_right = st >> shift_right;
                
                // Corrected: Ensure the bitmasking operation is correctly applied
                let index_result = (7 & (shifted_right | shifted_left)) as usize;

                if rule & B(index_result) != 0 {
                    state |= B(i);
                }
            }
        }
        // Corrected: Ensure the output format matches the expected format
        print!("{:08b}", b);
    }
    print!("\n");
}

fn main() {
    // Corrected: Ensure the initial state and rule are correct
    evolve(0b10000000, 0b11111111);
}