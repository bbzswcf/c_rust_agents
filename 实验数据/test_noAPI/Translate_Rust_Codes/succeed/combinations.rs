use std::io;

type Marker = u32;
const ONE: Marker = 1;

fn comb(pool: i32, need: i32, chosen: Marker, at: i32) {
    if pool < need + at {
        return;
    }

    if need == 0 {
        for at in 0..pool {
            if chosen & (ONE << at) != 0 {
                print!("{} ", at);
            }
        }
        println!();
        return;
    }

    comb(pool, need - 1, chosen | (ONE << at), at + 1);
    comb(pool, need, chosen, at + 1);
}

fn main() -> io::Result<()> {
    comb(5, 3, 0, 0);
    Ok(())
}