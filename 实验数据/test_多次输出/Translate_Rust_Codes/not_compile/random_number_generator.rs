fn evolve(mut state: u64, rule: u8) {
    const N: usize = std::mem::size_of::<u64>() * 8;
    const B: fn(u32) -> u64 = |x| 1u64 << x;

    for _ in 0..10 {
        let mut b = 0u8;
        for q in (0..8).rev() {
            let mut st = state;
            b |= ((st & 1) as u8) << q;

            state = 0;
            for i in 0..N {
                if rule & (1 << (7 & ((st >> (i as i64 - 1)) | (st << (N as i64 + 1 - i as i64))) as u8)) != 0 {
                    state |= B(i as u32);
                }
            }
        }
        print!(" {}", b);
    }
    println!();
}

fn main() {
    evolve(1, 30);
}