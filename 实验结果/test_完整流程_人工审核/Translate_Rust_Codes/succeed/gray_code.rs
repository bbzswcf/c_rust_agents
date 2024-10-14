fn gray_encode(n: i32) -> i32 {
    n ^ (n >> 1)
}

fn gray_decode(n: i32) -> i32 {
    let mut p = n;
    let mut n = n;
    while n != 0 {
        n >>= 1;
        p ^= n;
    }
    p
}

fn fmtbool(n: i32, buf: &mut [char]) {
    let mut b = buf.len() - 1;
    let mut n = n;
    while b > 0 { // Modified: Corrected loop condition to process all bits
        buf[b] = ('0' as u8 + (n & 1) as u8) as char;
        n >>= 1;
        b -= 1;
    }
    buf[b] = '\0'; // Modified: Set null terminator at the end of the buffer
}

fn main() {
    let mut bi = [' '; 6]; // Modified: Initialize with spaces instead of null terminators
    let mut bg = [' '; 6];
    let mut bb = [' '; 6];

    for i in 0..32 {
        let g = gray_encode(i);
        let b = gray_decode(g);
        fmtbool(i, &mut bi);
        fmtbool(g, &mut bg);
        fmtbool(b, &mut bb);
        // Modified: Convert character arrays to strings excluding the null terminator
        println!("{:2} : {:5} => {:5} => {:5} : {:2}", i, bi[1..].iter().collect::<String>(), bg[1..].iter().collect::<String>(), bb[1..].iter().collect::<String>(), b);
    }
}