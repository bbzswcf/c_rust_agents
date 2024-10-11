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

fn fmtbool(n: i32, buf: &mut [char; 6]) {
    let mut b = buf.len() - 1;
    buf[b] = '\0';
    b -= 1;
    let mut n = n;
    while b != 0 {
        buf[b] = '0' + (n & 1) as char;
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