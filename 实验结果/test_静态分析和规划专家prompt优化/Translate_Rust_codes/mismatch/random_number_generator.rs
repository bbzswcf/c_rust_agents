use std::io;
type ULL = u64;
const N: usize = std::mem::size_of::<ULL>() * 8;
const B: fn(usize) -> ULL = |x| 1 << x;

fn evolve(mut state: ULL, rule: u64) {
    for _p in 0..10 {
        let mut b = 0;
        for q in (0..8).rev() {
            let mut st = state;
            b |= ((st & 1) << q) as i32;
            let mut new_state = 0;
            for i in 0..N {
                let left = (st >> (i.saturating_sub(1) & (N - 1))) & 1;
                let center = (st >> i) & 1;
                let right = (st >> ((i + 1) & (N - 1))) & 1;
                let neighborhood = (left << 2) | (center << 1) | right;
                if rule & (1 << (7 & neighborhood)) != 0 {
                    new_state |= 1 << i;
                }
            }
            state = new_state;
        }
        print!(" {}", b);
    }
    println!();
}

fn main() {
    evolve(1, 30);
}