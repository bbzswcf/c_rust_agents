use std::env;
use std::fmt;
use std::io::{self, Write};
use std::num::Wrapping;
use std::process;

const S_SUITS: &[char] = &['♣', '♦', '♥', '♠'];
const S_NUMS: &[char] = &['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];

const RMAX32: u32 = (1 << 31) - 1;
static mut SEED: Wrapping<u32> = Wrapping(1);

fn rnd() -> u32 {
    unsafe {
        SEED = (SEED * Wrapping(214013) + Wrapping(2531011)) & Wrapping(RMAX32);
        (SEED >> 16).0
    }
}

fn srnd(x: u32) {
    unsafe {
        SEED = Wrapping(x);
    }
}

struct Card([u8; 52]);

impl Card {
    fn show(&self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for (i, &card) in self.0.iter().enumerate() {
            let suit = S_SUITS[(card % 4) as usize];
            let num = S_NUMS[(card / 4) as usize];
            write!(
                handle,
                "  \x1b[{}m{}\x1b[m{}",
                32 - (1 + card) % 4 / 2,
                suit,
                num
            )
            .unwrap();
            if (i + 1) % 8 == 0 || i + 1 == 52 {
                writeln!(handle).unwrap();
            }
        }
    }
}

fn deal(s: u32, t: &mut [u8; 52]) {
    srnd(s);
    for i in 0..52 {
        t[i] = (51 - i) as u8;
    }
    for i in 0..51 {
        let j = 51 - rnd() % (52 - i);
        t.swap(i, j as usize);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = match args.get(1) {
        Some(arg) => arg.parse().unwrap_or(11982),
        None => 11982,
    };

    if s <= 0 {
        process::exit(1);
    }

    let mut card = [0u8; 52];
    deal(s, &mut card);

    println!("Hand {}", s);
    Card(card).show();
}