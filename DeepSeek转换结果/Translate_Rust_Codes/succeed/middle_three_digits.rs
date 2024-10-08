fn mid3(n: i32) -> Option<&'static str> {
    static mut BUF: [u8; 32] = [0; 32];
    let abs_n = if n > 0 { n } else { -n };
    let formatted = format!("{}", abs_n);
    let len = formatted.len();

    if len < 3 || len % 2 == 0 {
        return None;
    }

    let mid = len / 2 - 1;
    unsafe {
        BUF[..len].copy_from_slice(formatted.as_bytes());
        BUF[mid + 3] = b'\0';
        Some(std::str::from_utf8_unchecked(&BUF[mid..mid + 3]))
    }
}

fn main() {
    let x = [
        123, 12345, 1234567, 987654321, 10001, -10001, -123, -100, 100, -12345, 1, 2, -1, -10, 2002,
        -2002, 0, 1234567890,
    ];

    for &num in &x {
        let m = mid3(num).unwrap_or("error");
        println!("{}: {}", num, m);
    }
}