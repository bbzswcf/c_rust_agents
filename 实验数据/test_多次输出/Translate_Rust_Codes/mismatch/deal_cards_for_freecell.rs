use std::env;
use std::io::{self, Write};
use std::process;
use std::fmt;

const S_SUITS: &[char] = &['♣', '♦', '♥', '♠'];
const S_NUMS: &[char] = &['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];

const RMAX32: u32 = (1 << 31) - 1;
static mut SEED: u32 = 1;

fn rnd() -> u32 {
    unsafe {
        SEED = (SEED * 214013 + 2531011) & RMAX32;
        SEED >> 16
    }
}

fn srnd(x: u32) {
    unsafe {
        SEED = x;
    }
}

fn show(cards: &[u8]) {
    let mut i = 0;
    for &card in cards {
        let suit = S_SUITS[(card % 4) as usize];
        let num = S_NUMS[(card / 4) as usize];
        print!("  \x1b[{}m{}\x1b[m{}", 32 - (1 + card) % 4 / 2, suit, num);
        i += 1;
        if i % 8 == 0 || i == 52 {
            println!();
        }
    }
}

fn deal(seed: u32, cards: &mut [u8; 52]) {
    srnd(seed);

    for i in 0..52 {
        cards[i] = (51 - i) as u8;
    }

    for i in 0..51 {
        let j = 51 - rnd() as usize % (52 - i);
        cards.swap(i, j);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let seed = match args.get(1) {
        Some(arg) => arg.parse().unwrap_or(11982),
        None => 11982,
    };

    if seed <= 0 {
        eprintln!("Seed must be a positive integer.");
        process::exit(1);
    }

    let mut cards = [0u8; 52];
    deal(seed, &mut cards);

    println!("Hand {}", seed);
    show(&cards);
}