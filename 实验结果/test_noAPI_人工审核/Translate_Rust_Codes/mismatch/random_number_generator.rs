use std::io::{self, Write};

type ULL = u64;
const N: usize = std::mem::size_of::<ULL>() * 8;
const B: fn(u32) -> ULL = |x| 1_u64 << x;

fn evolve(mut state: ULL, rule: i32) {
    for _p in 0..10 {
        let mut b: u8 = 0; // Ensure `b` is of type `u8` to match the bitwise operations
        for q in (0..8).rev() {
            let mut st = state;
            // Ensure type casting matches bitwise operations
            b |= ((st & 1) as u8) << q;

            state = 0; // Ensure `state` is correctly initialized
            for i in 0..N {
                // Correct shift logic to ensure it is within valid range
                let shift_amount = (i as i32 - 1).rem_euclid(N as i32) as u32;
                let shifted_st = st >> shift_amount;

                // Correct shift amount clamping logic
                let shift_amount_clamp = (N as i32 - i as i32 - 1).rem_euclid(N as i32) as u32;
                let shifted_state = (shifted_st | (st << shift_amount_clamp)) & 7;

                // Correctly apply the rule to the current state
                if rule & (1 << (shifted_state as i32)) != 0 { // Ensure type casting is correct
                    state |= B(i as u32); // Ensure type casting matches bitwise operations
                }
            }
        }
        print!(" {}", b);
        io::stdout().flush().unwrap();
    }
    println!();
}

fn main() {
    evolve(1, 30);
}