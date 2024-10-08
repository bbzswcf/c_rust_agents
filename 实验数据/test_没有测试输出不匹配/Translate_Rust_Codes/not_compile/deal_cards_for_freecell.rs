use std::env;
use std::ffi::CString;
use libc::setlocale; // Ensure libc crate is included in Cargo.toml

const S_SUITS: &str = "";
const S_NUMS: &str = "A23456789TJQK";
const RMAX32: u32 = (1 << 31) - 1;

static mut SEED: i32 = 1;

fn rnd() -> i32 {
    unsafe {
        // Ensure RMAX32 is cast to i32 before performing the bitwise AND operation
        SEED = (SEED * 214013 + 2531011) & RMAX32 as i32;
        SEED >> 16
    }
}

fn srnd(x: i32) {
    unsafe {
        SEED = x;
    }
}

fn show(c: &[i32]) {
    for (i, &card) in c.iter().enumerate() {
        print!("  \x1B[{}m{}\x1B[m{}", 32 - (1 + card) % 4 / 2,
               S_SUITS.chars().nth(card as usize % 4).unwrap(),
               S_NUMS.chars().nth(card as usize / 4).unwrap());
        if (i + 1) % 8 == 0 || i + 1 == 52 {
            println!();
        }
    }
}

fn deal(s: i32, t: &mut [i32; 52]) {
    srnd(s);

    for i in 0..52 {
        t[i] = 51 - i as i32; // Convert i to i32 before performing the subtraction
    }
    for i in 0..51 {
        let j = 51 - rnd() % (52 - i);
        t.swap(i as usize, j as usize); // Convert i and j to usize before calling swap
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = if args.len() < 2 || args[1].parse::<i32>().is_err() {
        11982
    } else {
        args[1].parse::<i32>().unwrap()
    };

    unsafe {
        // Ensure the libc crate is included in Cargo.toml and the import path is correct
        setlocale(libc::LC_ALL, CString::new("").unwrap().as_ptr());
    }

    let mut card = [0; 52];
    deal(s, &mut card);
    println!("Hand {}", s);
    show(&card);
}