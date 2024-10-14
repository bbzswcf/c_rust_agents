use std::env;
use std::fmt;
use std::num::NonZeroU64;

fn digit_sum(n: u64, sum: u64) -> u64 {
    let mut sum = sum + 1;
    let mut n = n;
    while n > 0 && n % 10 == 0 {
        sum -= 9;
        n /= 10;
    }
    sum
}

fn divisible(n: u64, d: u64) -> bool {
    if d & 1 == 0 && n & 1 == 1 {
        return false;
    }
    n % d == 0
}

struct NumberFormatter(u64);

impl fmt::Display for NumberFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.to_string();
        let mut formatted = String::new();
        let mut count = 0;
        for c in s.chars().rev() {
            if count == 3 {
                formatted.push(',');
                count = 0;
            }
            formatted.push(c);
            count += 1;
        }
        write!(f, "{}", formatted.chars().rev().collect::<String>())
    }
}

fn main() {
    env::set_var("LC_NUMERIC", "en_US.UTF-8");

    let mut previous = 1;
    let mut gap = 0;
    let mut sum = 0;
    let mut niven_index = 0;
    let mut gap_index = 1;

    println!("Gap index  Gap    Niven index    Niven number");
    for niven in 1.. {
        sum = digit_sum(niven, sum);
        if divisible(niven, sum) {
            if niven > previous + gap {
                gap = niven - previous;
                println!(
                    "{:9} {:4} {:14} {:15}",
                    gap_index,
                    NumberFormatter(gap),
                    niven_index,
                    NumberFormatter(previous)
                );
                gap_index += 1;
                if gap_index > 20 {
                    break;
                }
            }
            previous = niven;
            niven_index += 1;
        }
    }
}