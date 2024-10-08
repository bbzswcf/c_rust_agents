fn gray_encode(n: i32) -> i32 {
    n ^ (n >> 1)
}

fn gray_decode(mut n: i32) -> i32 {
    // 修改: 初始化 p 为 n
    let mut p = n;
    while n != 0 {
        n >>= 1;
        p ^= n;
    }
    p
}

fn fmtbool(mut n: i32, buf: &mut [char; 6]) {
    // 修改: 初始化 b 为 buf.len() - 1
    let mut b = buf.len() - 1;
    // 修改: 使用 buf.len() - 1 而不是硬编码的 5
    b = buf.len() - 1;
    // 修改: 使用 while b >= 0 以确保循环覆盖所有位
    while b >= 0 {
        // 修改: 使用 if 语句来处理 n & 1 的情况
        if n & 1 == 1 {
            buf[b] = '1';
        } else {
            buf[b] = '0';
        }
        n >>= 1;
        b -= 1;
    }
}

fn main() {
    let mut bi = ['0'; 6]; // 修改: 使用 '0' 而不是 ' '
    let mut bg = ['0'; 6]; // 修改: 使用 '0' 而不是 ' '
    let mut bb = ['0'; 6]; // 修改: 使用 '0' 而不是 ' '

    for i in 0..32 {
        let g = gray_encode(i);
        let b = gray_decode(g);
        fmtbool(i, &mut bi);
        fmtbool(g, &mut bg);
        fmtbool(b, &mut bb);
        print!("{:2} : {:5} => {:5} => {:5} : {:2}\n", i, bi.iter().collect::<String>(), bg.iter().collect::<String>(), bb.iter().collect::<String>(), b);
    }
}