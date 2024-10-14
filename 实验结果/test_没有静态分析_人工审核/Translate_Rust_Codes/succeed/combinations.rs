fn comb(pool: i32, need: i32, chosen: u64, at: i32) {
    if pool < need + at {
        return;
    }

    if need == 0 {
        for at in 0..pool {
            if chosen & (1 << at) != 0 {
                print!("{} ", at);
            }
        }
        println!();
        return;
    }

    comb(pool, need - 1, chosen | (1 << at), at + 1);
    comb(pool, need, chosen, at + 1);
}

fn main() {
    comb(5, 3, 0, 0);
}