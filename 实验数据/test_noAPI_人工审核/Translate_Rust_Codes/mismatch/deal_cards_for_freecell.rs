use std::env;
use std::io::{self, Write};
use std::os::raw::c_int;
use std::sync::atomic::{AtomicI32, Ordering};

// Modified: Corrected the Unicode characters for the suits
const S_SUITS: &[char] = &['\u{2663}', '\u{2666}', '\u{2665}', '\u{2660}'];
const S_NUMS: &[char] = &['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];
const RMAX32: i32 = i32::MAX; // Modified: Changed type to `i32`

// Modified: Used `AtomicI32` for thread-safe access to `SEED`
static SEED: AtomicI32 = AtomicI32::new(1);

fn rnd() -> c_int {
    let seed = SEED.fetch_add(1, Ordering::SeqCst); // Modified: Used atomic fetch_add for thread-safe increment
    ((seed as i64 * 214013 + 2531011) & (RMAX32 as i64)) as i32 >> 16
}

fn srnd(x: c_int) {
    // Modified: Ensure the seed value is within the valid range for the random number generator
    let seed_value = if x < 0 {
        -x
    } else {
        x
    };
    SEED.store(seed_value, Ordering::SeqCst); // Modified: Used atomic store for thread-safe assignment
}

fn show(c: &[c_int; 52]) {
    let mut i = 0;
    for &card in c {
        // Modified: Corrected the color formatting logic to ensure correct color codes
        let color = match card % 4 {
            0 | 1 => 31, // Red suits
            2 | 3 => 30, // Black suits
            _ => unreachable!(),
        };
        print!("  \x1B[{}m{}\x1B[{}m{}", color, S_SUITS[(card % 4) as usize], color, S_NUMS[(card / 4) as usize]);
        i += 1;
        if i % 8 == 0 || i == 52 {
            println!();
        }
    }
}

fn deal(s: c_int, t: &mut [c_int; 52]) {
    srnd(s);

    for i in 0..52 {
        t[i] = i as i32;
    }

    // Modified: Optimized the shuffling algorithm to reduce the number of operations
    for i in (1..52).rev() {
        let j = rnd() as usize % (i + 1);
        t.swap(i, j);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let s = if args.len() < 2 {
        11982
    } else {
        match args[1].parse::<c_int>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Invalid argument provided. Using default seed.");
                11982
            }
        }
    };

    let mut card = [0; 52];

    deal(s, &mut card);
    println!("Hand {}", s);
    show(&card);

    Ok(())
}