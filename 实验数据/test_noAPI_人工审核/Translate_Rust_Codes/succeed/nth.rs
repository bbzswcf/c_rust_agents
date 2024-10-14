use std::fmt::Write;

fn add_suffix(num: i32, buf: &mut String) {
    let suffixes = ["th", "st", "nd", "rd"];
    let i = match num % 10 {
        1 if num % 100 != 11 => 1,
        2 if num % 100 != 12 => 2,
        3 if num % 100 != 13 => 3,
        _ => 0,
    };

    write!(buf, "{}{}", num, suffixes[i]).unwrap();
}

fn main() {
    println!("Set [0,25]:");
    for i in 0..26 {
        let mut s = String::with_capacity(5);
        add_suffix(i, &mut s);
        print!("{} ", s);
    }
    println!();

    println!("Set [250,265]:");
    for i in 250..266 {
        let mut s = String::with_capacity(6);
        add_suffix(i, &mut s);
        print!("{} ", s);
    }
    println!();

    println!("Set [1000,1025]:");
    for i in 1000..1026 {
        let mut s = String::with_capacity(7);
        add_suffix(i, &mut s);
        print!("{} ", s);
    }
    println!();
}