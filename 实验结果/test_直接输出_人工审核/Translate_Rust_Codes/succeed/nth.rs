fn add_suffix(num: i32, buf: &mut [u8]) -> &str {
    let suffixes = ["th", "st", "nd", "rd"];
    let i = match num % 10 {
        1 if num % 100 != 11 => 1,
        2 if num % 100 != 12 => 2,
        3 if num % 100 != 13 => 3,
        _ => 0,
    };

    let formatted = format!("{}{}", num, suffixes[i]);
    let len = formatted.len();
    buf[..len].copy_from_slice(formatted.as_bytes());
    std::str::from_utf8(&buf[..len]).unwrap()
}

fn main() {
    println!("Set [0,25]:");
    for i in 0..26 {
        let mut s = [0u8; 5];
        print!("{} ", add_suffix(i, &mut s));
    }
    println!();

    println!("Set [250,265]:");
    for i in 250..266 {
        let mut s = [0u8; 6];
        print!("{} ", add_suffix(i, &mut s));
    }
    println!();

    println!("Set [1000,1025]:");
    for i in 1000..1026 {
        let mut s = [0u8; 7];
        print!("{} ", add_suffix(i, &mut s));
    }
    println!();
}