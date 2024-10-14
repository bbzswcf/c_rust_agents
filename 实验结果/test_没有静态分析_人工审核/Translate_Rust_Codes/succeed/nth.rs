fn add_suffix(num: i32, buf: &mut [u8]) -> &[u8] {
    let suffixes = ["th", "st", "nd", "rd"];
    let i = match num % 10 {
        1 if num % 100 != 11 => 1,
        2 if num % 100 != 12 => 2,
        3 if num % 100 != 13 => 3,
        _ => 0,
    };

    let suffix = suffixes[i].as_bytes();
    let num_str = num.to_string();
    let num_bytes = num_str.as_bytes();

    let total_len = num_bytes.len() + suffix.len();
    if total_len > buf.len() {
        panic!("Buffer too small");
    }

    buf[..num_bytes.len()].copy_from_slice(num_bytes);
    buf[num_bytes.len()..total_len].copy_from_slice(suffix);
    &buf[..total_len]
}

fn main() {
    println!("Set [0,25]:");
    for i in 0..26 {
        let mut s = [0u8; 5];
        let result = add_suffix(i, &mut s);
        print!("{} ", std::str::from_utf8(result).unwrap());
    }
    println!();

    println!("Set [250,265]:");
    for i in 250..266 {
        let mut s = [0u8; 6];
        let result = add_suffix(i, &mut s);
        print!("{} ", std::str::from_utf8(result).unwrap());
    }
    println!();

    println!("Set [1000,1025]:");
    for i in 1000..1026 {
        let mut s = [0u8; 7];
        let result = add_suffix(i, &mut s);
        print!("{} ", std::str::from_utf8(result).unwrap());
    }
    println!();
}