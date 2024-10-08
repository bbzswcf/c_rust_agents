fn comb(pool: i32, need: i32, chosen: u64, at: i32) {
    if pool < need + at {
        return; /* not enough bits left */
    }

    if need == 0 {
        /* got all we needed; print the thing. */
        for at in 0..pool {
            if chosen & (1 << at) != 0 {
                print!("{} ", at);
            }
        }
        println!();
        return;
    }

    /* if we choose the current item, "or" (|) the bit to mark it so. */
    comb(pool, need - 1, chosen | (1 << at), at + 1);
    comb(pool, need, chosen, at + 1);  /* or don't choose it, go to next */
}

fn main() {
    comb(5, 3, 0, 0);
}