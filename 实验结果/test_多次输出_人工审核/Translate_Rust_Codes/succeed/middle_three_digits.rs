fn mid3(n: i32) -> Option<&'static str> {
    static mut BUF: [u8; 32] = [0; 32];
    let abs_n = n.abs();
    let len = unsafe {
        std::ptr::copy_nonoverlapping(format!("{}", abs_n).as_bytes().as_ptr(), BUF.as_mut_ptr(), format!("{}", abs_n).len());
        BUF[format!("{}", abs_n).len()] = 0; // Null-terminate the string
        format!("{}", abs_n).len()
    };

    if len < 3 || len % 2 == 0 {
        return None;
    }

    let mid = len / 2 - 1;
    unsafe {
        BUF[mid + 3] = 0; // Null-terminate the substring
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