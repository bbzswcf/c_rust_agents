use std::io;

type ULL = u64;
const N: usize = std::mem::size_of::<ULL>() * 8;
const B: fn(usize) -> ULL = |x| 1_u64 << x;

fn evolve(mut state: ULL, rule: i32) {
    for _p in 0..10 {
        let mut b = 0;
        for q in (0..8).rev() {
            let mut st = state;
            b |= ((st & 1) << q) as i32;

            state = 0;
            for i in 0..N {
                if rule & (B(7 & ((st >> (i as isize - 1)) | (st << (N as isize + 1 - i as isize))) as usize)) != 0 {
                    state |= B(i);
                }
            }
        }
        print!(" {}", b);
    }
    println!();
}

fn main() -> io::Result<()> {
    evolve(1, 30);
    Ok(())
}