fn t(n: i32) {
    let mut i = n * (n - 1) / 2;
    let mut len = 1;
    let mut c = 1;

    while c < i {
        c *= 10;
        len += 1;
    }
    c -= i; // c is the col where width changes

    const SPEED_MATTERS: bool = false;

    if SPEED_MATTERS {
        // 修改: 将 `len` 转换为 `usize` 以匹配 `format!` 的参数类型
        let mut tmp = format!("{:0width$}", 0, width = len as usize);
        // 修改: 将 `4096` 转换为 `usize` 以匹配 `String::with_capacity` 的参数类型
        let mut s = String::with_capacity(4096 as usize);
        // 修改: 将 `p` 的类型从 `&mut String` 改为 `&mut str` 以匹配 `replace_range` 的参数类型
        let mut p: &mut str = s.as_mut_str(); // 修改: 使用 `as_mut_str` 获取可变字符串切片

        fn inc_numstr(tmp: &mut String, len: usize) {
            // 修改: 将 `k` 的类型从 `usize` 改为 `usize` 以匹配索引操作
            let mut k: usize = len;
            loop {
                if k == 0 {
                    return;
                }
                k -= 1;

                if tmp.as_bytes()[k] == b'9' {
                    // 修改: 避免不必要的 clone，直接修改字符串
                    tmp.replace_range(k..=k, "0");
                    continue;
                }

                if tmp.as_bytes()[k] == b'!' {
                    // 修改: 避免不必要的 clone，直接修改字符串
                    tmp.replace_range(k..=k, "1");
                } else {
                    // 修改: 避免不必要的 clone，直接修改字符串
                    let mut bytes = tmp.as_bytes();
                    bytes[k] += 1;
                    tmp.replace_range(k..=k, std::str::from_utf8(&bytes[k..=k]).unwrap());
                }
                break;
            }
        }

        for i in 1..=n {
            for j in 1..=i {
                inc_numstr(&mut tmp, len);
                let src = tmp.as_bytes();
                let copy_len = len - (j < c) as usize;
                // 修改: 将 `&[u8]` 转换为 `&str` 以匹配 `replace_range` 的参数类型
                let src_str = std::str::from_utf8(&src[(1 - (j >= c) as usize)..(1 - (j >= c) as usize) + copy_len]).unwrap();
                p.replace_range(..copy_len, src_str);

                // 修改: 使用 `push_str` 而不是 `String::push_str`
                if i - j != 0 {
                    p.push_str(" ");
                } else {
                    p.push_str("\n");
                }

                if p.len() + len >= 4096 {
                    print!("{}", s);
                    s.clear();
                    p = s.as_mut_str();
                }
            }
        }

        print!("{}", s);
    } else {
        let mut num = 1;
        for i in 1..=n {
            for j in 1..=i {
                print!("{:0width$}{}{}", num, if i - j != 0 { ' ' } else { '\n' }, width = len - (j < c) as usize);
                num += 1;
            }
        }
    }
}

fn main() {
    t(5);
    t(14);
}