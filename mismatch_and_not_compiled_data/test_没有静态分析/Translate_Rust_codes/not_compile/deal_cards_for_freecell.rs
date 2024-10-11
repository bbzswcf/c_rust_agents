extern crate libc; // Ensure the `libc` crate is properly imported
use std::env;
use std::ffi::CString;
use libc::setlocale;

const S_SUITS: &str = "";
const S_NUMS: &str = "A23456789TJQK";
const RMAX32: u32 = (1 << 31) - 1;
static mut SEED: i32 = 1;

fn rnd() -> i32 {
    unsafe {
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
    let mut i = 0;
    for &card in c {
        print!("  \x1b[{}m{}\x1b[m{}", 32 - (1 + card) % 4 / 2,
               S_SUITS.chars().nth(card as usize % 4).unwrap(),
               S_NUMS.chars().nth(card as usize / 4).unwrap());
        i += 1;
        if i % 8 == 0 || i == 52 {
            println!();
        }
    }
}

fn deal(s: i32, t: &mut [i32; 52]) {
    srnd(s);

    for i in 0..52 {
        t[i] = 51 - i as i32; // Convert `i` to `i32` to match the type of `t`
    }
    for i in 0..51 {
        let j = 51 - rnd() % (52 - i);
        t.swap(i as usize, j as usize); // Convert `i` and `j` to `usize` to match the expected type
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = if args.len() < 2 {
        11982
    } else {
        args[1].parse::<i32>().unwrap_or(11982)
    };

    // Ensure that the `setlocale` function is used correctly and safely
    // Handle potential errors and ensure the `CString` is not dropped prematurely
    let locale = CString::new("").unwrap();
    let _keep_alive = locale.clone(); // Keep the CString alive
    unsafe {
        setlocale(libc::LC_ALL, locale.as_ptr());
    }

    let mut card = [0; 52];
    deal(s, &mut card);
    println!("Hand {}", s);
    show(&card);
}