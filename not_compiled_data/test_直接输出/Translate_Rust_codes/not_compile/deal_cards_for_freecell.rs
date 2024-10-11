use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
use libc::setlocale;

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

fn show(c: &[i32; 52]) {
    for (i, &card) in c.iter().enumerate() {
        print!("  \x1B[{}m{}\x1B[m{}", 32 - (1 + card) % 4 / 2,
               ['♣', '♦', '♥', '♠'][(card % 4) as usize],
               ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'][(card / 4) as usize]);
        if (i + 1) % 8 == 0 || i + 1 == 52 {
            println!();
        }
    }
}

fn deal(s: i32, t: &mut [i32; 52]) {
    srnd(s);

    for i in 0..52 {
        t[i] = 51 - i;
    }
    for i in 0..51 {
        let j = 51 - rnd() % (52 - i);
        t.swap(i, j);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let s = if args.len() < 2 {
        11982
    } else {
        args[1].parse::<i32>()?
    };

    unsafe {
        setlocale(libc::LC_ALL, CString::new("").unwrap().as_ptr());
    }

    let mut card = [0; 52];
    deal(s, &mut card);
    println!("Hand {}", s);
    show(&card);

    Ok(())
}