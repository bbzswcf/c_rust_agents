fn main() {
    const N: usize = 2200;
    const N2: usize = 2200 * 2200 * 2;

    let mut r = vec![0; N + 1];
    let mut ab = vec![0; N2 + 1];

    for a in 1..=N {
        let a2 = a * a;
        for b in a..=N {
            let index = a2 + b * b;
            // 修改: 确保 index 在 ab 的范围内
            if index <= N2 {
                ab[index] = 1;
            }
        }
    }

    let mut s = 3;
    for c in 1..=N {
        let mut s1 = s;
        s += 2;
        let mut s2 = s;
        for d in (c + 1)..=N {
            // 修改: 确保 s1 在 ab 的范围内
            if s1 <= N2 {
                if ab[s1] == 1 {
                    r[d] = 1;
                }
            }
            s1 += s2;
            s2 += 2;
        }
    }

    for d in 1..=N {
        // 修改: 确保 d 在 r 的范围内
        if d <= N {
            if r[d] == 0 {
                print!("{} ", d);
            }
        }
    }
    println!();
}