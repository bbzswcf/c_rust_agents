use std::io::{self, Write};

fn t(n: i32) {
    let i = n * (n - 1) / 2;
    let mut len = 1;
    let mut c = 1;
    while c < i {
        c *= 10;
        len += 1;
    }
    c -= i; // c is the col where width changes

    let mut num = i + 1;
    for i in 1..=n {
        for j in 1..=i {
            print!("{:1$}", num, len - (j < c) as usize);
            num += 1;
            if i - j > 0 {
                print!(" ");
            } else {
                println!();
            }
        }
    }
}

fn main() {
    t(5);
    t(14);
}