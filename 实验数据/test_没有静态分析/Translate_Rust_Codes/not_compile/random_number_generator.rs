use std::io;

type ULL = u64;
const N: usize = std::mem::size_of::<ULL>() * 8;
const B: fn(usize) -> ULL = |x| 1_u64 << x;

fn evolve(mut state: ULL, rule: i32) {
    for _p in 0..10 {
        let mut b = 0;
        for q in (0..8).rev() {
            // Correct bit manipulation logic
            b |= ((state >> q) & 1) as i32;
        }
        // Remove state reset
        for i in 0..N {
            let shift_left = (N + 1 - i) % N; // Simplify shift logic
            let shift_right = i.checked_sub(1).unwrap_or(N - 1) % N; // Avoid subtraction overflow
            // Correct bitwise operations
            if (rule as u64) & (1 << ((state >> shift_right) | (state << shift_left)) as usize) != 0 {
                state |= B(i);
            }
        }
        print!(" {}", b);
    }
    println!();
}

fn main() -> io::Result<()> {
    // Correct state initialization
    evolve(0b10000000, 30);
    Ok(())
}