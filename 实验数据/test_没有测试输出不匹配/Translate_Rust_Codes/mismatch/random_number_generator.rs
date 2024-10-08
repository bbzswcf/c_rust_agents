use std::mem;
use std::cmp::min;

type ULL = u64;
const N: usize = mem::size_of::<ULL>() * 8;
const B: fn(usize) -> ULL = |x| 1 << x;

fn evolve(mut state: ULL, rule: i32) {
    for _p in 0..10 {
        let mut b: ULL = 0; // Modified: Changed type of `b` to `u64` to match the type of the expression
        for q in (0..8).rev() {
            let mut st = state;
            b |= (st & 1) << q; // Modified: Removed unnecessary cast to `i32`

            state = 0;
            for i in 0..N {
                // Modified: Ensure the shift amount does not exceed the bit width of `u64`
                let shift_left = min(63, (N as isize + 1 - i as isize).max(0) as u32); // Ensured non-negative shift amount
                let shift_right = min(63, (i as isize - 1).max(0) as u32); // Ensured non-negative shift amount
                if (rule as u64) & (B(7 & ((st >> shift_right) | (st << shift_left)) as usize)) != 0 {
                    state |= B(i);
                }
            }
        }
        print!(" {}", b);
    }
    print!("\n");
}

fn main() {
    evolve(1, 30);
}