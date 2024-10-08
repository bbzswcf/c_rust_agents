use std::io::{self, Write};

type ULL = u64;
const N: usize = std::mem::size_of::<ULL>() * 8;
const B: fn(usize) -> ULL = |x| 1_u64 << x;

fn evolve(mut state: ULL, rule: i32) {
    let mut b;
    let mut st;

    for _p in 0..10 {
        b = 0;
        // Modified: Correct loop bounds for `q` to iterate over the bits of the state
        for q in 0..N {
            st = state;
            b |= (st & 1) << q;

            state = 0;
            for i in 0..N {
                // Modified: Correctly calculate shift amount to ensure it does not exceed the bit width of ULL
                let shift_amount = (N as u32 - i as u32 - 1).min(63);
                let shifted_left = st << shift_amount;

                // Modified: Correctly calculate shift amount to handle the shift correctly
                let shift_amount = if i == 0 { 0 } else { (N as u32 - i as u32 - 1).min(63) };
                let shifted_right = st >> shift_amount;

                let combined = shifted_left | shifted_right;
                let masked = 7 & combined;
                // Modified: Correctly apply the rule using bitwise operation
                if (rule as u64) & (1 << masked) != 0 {
                    state |= B(i);
                }
            }
        }
        print!(" {}", b);
    }
    // Modified: Ensure the output is correctly flushed
    io::stdout().flush().unwrap();
    println!();
}

fn main() {
    evolve(1_u64, 30_i32); // No change needed here as types are correct
}