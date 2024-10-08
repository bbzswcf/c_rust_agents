// 修改: 在Cargo.toml中添加libc依赖
// [dependencies]
// libc = "0.2"

extern crate libc;
use libc::setlocale;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::sync::Mutex;

// 修改: 将S_SUITS和S_NUMS改为常量字符串
const S_SUITS: &str = "SHDC";
const S_NUMS: &str = "A23456789TJQK";
const RMAX32: u32 = (1 << 31) - 1;

static SEED: Mutex<i32> = Mutex::new(1);

fn rnd() -> i32 {
    let mut seed = SEED.lock().unwrap();
    *seed = (*seed * 214013 + 2531011) & RMAX32 as i32;
    *seed >> 16
}

fn srnd(x: i32) {
    let mut seed = SEED.lock().unwrap();
    *seed = x;
}

#[derive(Debug)]
struct Card {
    suit: char,
    num: char,
}

fn show(c: &[i32; 52]) {
    for (i, &card) in c.iter().enumerate() {
        let suit = S_SUITS.chars().nth(card as usize % 4).unwrap_or('?');
        let num = S_NUMS.chars().nth(card as usize / 4).unwrap_or('?');
        let card = Card { suit, num };
        print!("  \x1b[{}m{:?}\x1b[m", 32 - (1 + card.num as i32) % 4 / 2, card);
        if (i + 1) % 8 == 0 || i + 1 == 52 {
            println!();
        }
    }
}

fn deal(s: i32, t: &mut [i32; 52]) {
    srnd(s);

    for i in 0..52 {
        t[i] = 51 - i as i32;
    }
    for i in 0..51 {
        let j = 51 - rnd() % (52 - i);
        let temp = t[i as usize];
        t[i as usize] = t[j as usize];
        t[j as usize] = temp;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // 修改: 使用unwrap_or_else处理CString创建失败的情况
    let locale = CString::new("").unwrap_or_else(|err| {
        eprintln!("Failed to create CString: {}", err);
        std::process::exit(1);
    });

    unsafe {
        setlocale(libc::LC_ALL, locale.as_ptr());
    }

    // 修改: 处理命令行参数解析失败的情况
    let s = match args.get(1) {
        Some(arg) => match arg.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid argument: expected an integer");
                std::process::exit(1);
            }
        },
        None => 11982,
    };

    let mut card = [0; 52];
    deal(s, &mut card);
    println!("Hand {}", s);
    show(&card);
}