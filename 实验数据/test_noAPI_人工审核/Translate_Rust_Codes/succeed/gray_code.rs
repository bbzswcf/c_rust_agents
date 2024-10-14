fn gray_encode(n: i32) -> i32 {
    n ^ (n >> 1)
}

fn gray_decode(mut n: i32) -> i32 {
    let mut p = n;
    while n > 0 {
        n >>= 1;
        p ^= n;
    }
    p
}

fn fmtbool(mut n: i32, buf: &mut [char; 6]) {
    // Modified: Changed array length check to `if buf.len() < 2` to ensure the array has at least two elements
    if buf.len() < 2 {
        return;
    }
    let mut b = (buf.len() - 1) as i32; // Modified: Changed type of `b` to `i32` to allow the loop to terminate correctly
    buf[buf.len() - 1] = '\0'; // Modified: Correctly placed the null character at the last position of the array
    b -= 1;
    while b >= 0 { // Modified: Changed loop condition to `while b >= 0` to ensure the loop iterates correctly
        buf[b as usize] = ('0' as u8 + (n & 1) as u8) as char;
        n >>= 1;
        b -= 1;
    }
}

fn main() {
    let mut bi = ['\0'; 6];
    let mut bg = ['\0'; 6];
    let mut bb = ['\0'; 6];

    for i in 0..32 {
        let g = gray_encode(i);
        let b = gray_decode(g);
        fmtbool(i, &mut bi);
        fmtbool(g, &mut bg);
        fmtbool(b, &mut bb);
        println!("{:2} : {:5} => {:5} => {:5} : {:2}", i, bi.iter().collect::<String>(), bg.iter().collect::<String>(), bb.iter().collect::<String>(), b);
    }
}