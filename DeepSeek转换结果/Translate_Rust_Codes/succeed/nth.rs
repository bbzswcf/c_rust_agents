use std::io::Write;

fn add_suffix(num: i32, buf: &mut Vec<u8>) -> Result<&[u8], &'static str> {
    let suffixes: &'static [&'static str] = &["th", "st", "nd", "rd"]; // 
    let i = match num % 10 {
        1 if num % 100 == 11 => 0,
        2 if num % 100 == 12 => 0,
        3 if num % 100 == 13 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        _ => 0,
    };

    // :  &mut *buf  Cursor,  buf
    let mut cursor = std::io::Cursor::new(&mut *buf);
    write!(cursor, "{}{}", num, suffixes[i]).map_err(|_| "Failed to write to buffer")?;

    // 修改: 使用 buf 而不是 cursor.get_ref()，因为 cursor.get_ref() 返回的是 buf 的引用
    // 直接返回 buf 的切片，而不是通过 cursor 获取引用
    Ok(&buf[..])
}

fn main() {
    print!("Set [0,25]:\n");
    let mut s = Vec::new(); //  Vec<u8> 
    for i in 0..26 {
        s.clear(); //  Vec<u8> 
        match add_suffix(i, &mut s) {
            Ok(result) => match std::str::from_utf8(result) {
                Ok(str_result) => print!("{} ", str_result),
                Err(_) => print!("Invalid UTF-8 sequence "),
            },
            Err(_) => print!("Failed to write to buffer "),
        }
    }
    print!("\n");

    print!("Set [250,265]:\n");
    s.clear(); //  Vec<u8> 
    for i in 250..266 {
        s.clear(); //  Vec<u8> 
        match add_suffix(i, &mut s) {
            Ok(result) => match std::str::from_utf8(result) {
                Ok(str_result) => print!("{} ", str_result),
                Err(_) => print!("Invalid UTF-8 sequence "),
            },
            Err(_) => print!("Failed to write to buffer "),
        }
    }
    print!("\n");

    print!("Set [1000,1025]:\n");
    s.clear(); //  Vec<u8> 
    for i in 1000..1026 {
        s.clear(); //  Vec<u8> 
        match add_suffix(i, &mut s) {
            Ok(result) => match std::str::from_utf8(result) {
                Ok(str_result) => print!("{} ", str_result),
                Err(_) => print!("Invalid UTF-8 sequence "),
            },
            Err(_) => print!("Failed to write to buffer "),
        }
    }
    print!("\n");
}