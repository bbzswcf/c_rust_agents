use std::env;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_char;
const S_SUITS: &[u32] = &[0x2663, 0x2666, 0x2665, 0x2660];
const S_NUMS: &[u32] = &[0x41, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x54, 0x4A, 0x51, 0x4B];
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
fn show(c: &[u32]) {
    for (i, &card) in c.iter().enumerate() {
        let suit = S_SUITS[(card % 4) as usize];
        let num = S_NUMS[(card / 4) as usize];
        print!("  \x1B[{}m{}\x1B[m{}", 32 - (1 + card) % 4 / 2, char::from_u32(suit).unwrap(), char::from_u32(num).unwrap());
        if (i + 1) % 8 == 0 || i + 1 == 52 {
            println!();
        }
    }
}
fn deal(s: u32, t: &mut [u32; 52]) {
    srnd(s);
    for i in 0..52 {
        t[i] = (51 - i) as u32;
    }
    for i in 0..51 {
        let j = 51 - (rnd() % (52 - i)) as usize;
        t.swap(i, j.try_into().unwrap());
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let s: u32 = if args.len() < 2 {
        11982
    } else {
        args[1].parse().unwrap_or(11982)
    };
    
    let mut card = [0u32; 52];
    deal(s, &mut card);
    println!("Hand {}", s);
    show(&card);
}