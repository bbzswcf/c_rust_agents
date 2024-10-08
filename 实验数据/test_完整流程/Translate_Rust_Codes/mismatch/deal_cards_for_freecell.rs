use std::cell::Cell;
use std::env;

const RMAX32: u32 = (1 << 31) - 1;
thread_local!(static SEED: Cell<u32> = Cell::new(1));

fn rnd() -> u32 {
    SEED.with(|seed| {
        let new_seed = (seed.get().checked_mul(214013).unwrap_or(0) + 2531011) & RMAX32;
        seed.set(new_seed);
        new_seed >> 16
    })
}

fn srnd(x: u32) {
    SEED.with(|seed| {
        seed.set(x);
    });
}

fn show(c: &[i32]) {
    let s_suits = "HDSC"; // Hearts, Diamonds, Spades, Clubs
    let s_nums = "A23456789TJQK";
    let mut i = 0;

    for &card in c {
        let suit_char = s_suits.chars().nth((card % 4) as usize).unwrap_or('?');
        let num_char = s_nums.chars().nth((card / 4) as usize).unwrap_or('?');
        let color = match (card % 4) as usize {
            0 | 1 => 31, // Red for Hearts (H) and Diamonds (D)
            2 | 3 => 30, // Black for Spades (S) and Clubs (C)
            _ => 0,
        };
        print!("  \x1b[{}m{}{}", color, num_char, suit_char); // Corrected: Ensure correct card display format
        i += 1;
        if i % 8 == 0 || i == 52 {
            println!();
        }
    }
}

fn deal(s: u32, t: &mut [i32; 52]) {
    srnd(s);

    for i in 0..52 {
        t[i] = i as i32; // Corrected: Initialize cards in order
    }

    for i in 0..51 {
        let j = rnd() as usize % (52 - i); // Corrected: Shuffle in correct order
        t.swap(i, i + j);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s: u32 = match args.get(1).and_then(|arg| arg.parse().ok()) {
        Some(seed) => seed,
        None => {
            eprintln!("Invalid seed provided, using default value 11982");
            11982
        }
    };

    let mut card = [0; 52];
    deal(s, &mut card);
    println!("Hand {}", s);
    show(&card);
}