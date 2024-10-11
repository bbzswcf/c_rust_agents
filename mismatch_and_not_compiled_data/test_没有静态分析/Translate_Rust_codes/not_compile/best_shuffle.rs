use std::mem;
use std::process;

fn best_shuffle(txt: &str, result: &mut [char]) {
    let len = txt.len();
    if len == 0 {
        return;
    }

    // Ensure the length of `result` is equal to `txt.len()`
    debug_assert_eq!(len, result.len());

    let mut counts = [0; u8::MAX as usize + 1];
    let mut fmax = 0;
    for &ch in txt.as_bytes() {
        counts[ch as usize] += 1;
        if fmax < counts[ch as usize] {
            fmax = counts[ch as usize];
        }
    }
    debug_assert!(fmax > 0 && fmax <= len);

    let mut ndx1 = vec![0; len];
    let mut i = 0;
    for ch in 0..=u8::MAX {
        if counts[ch as usize] > 0 {
            for (j, &c) in txt.as_bytes().iter().enumerate() {
                if ch == c {
                    ndx1[i] = j;
                    i += 1;
                }
            }
        }
    }

    let mut ndx2 = vec![0; len];
    let mut n = 0;
    let mut m = 0;
    for i in 0..len {
        // Ensure `n` is a valid index for `ndx1`
        if n < len {
            ndx2[i] = ndx1[n];
            n += fmax;
            if n >= len {
                m += 1;
                n = m;
            }
        } else {
            panic!("Index out of bounds in `ndx1` initialization");
        }
    }

    let grp = 1 + (len - 1) / fmax;
    debug_assert!(grp > 0 && grp <= len);

    let lng = 1 + (len - 1) % fmax;
    debug_assert!(lng > 0 && lng <= len);

    for i in 0..fmax {
        let first = ndx2[i * grp];
        let glen = grp - (i < lng) as usize;
        for k in 1..glen {
            // Ensure `i * grp + k` is a valid index for `ndx2`
            // and `i * grp + k - 1` is a valid index for `ndx1`
            if i * grp + k < len && i * grp + k - 1 < len {
                ndx1[i * grp + k - 1] = ndx2[i * grp + k];
            } else {
                panic!("Index out of bounds in `ndx1` reassignment");
            }
        }
        // Ensure `i * grp + glen - 1` is a valid index for `ndx1`
        if i * grp + glen - 1 < len {
            ndx1[i * grp + glen - 1] = first;
        } else {
            panic!("Index out of bounds in `ndx1` reassignment");
        }
    }

    for i in 0..len {
        // Ensure `ndx1[i]` is a valid index for `txt`
        if ndx1[i] < len {
            if let Some(ch) = txt.chars().nth(ndx1[i]) {
                result[ndx2[i]] = ch;
            } else {
                panic!("Index out of bounds in `txt`");
            }
        } else {
            panic!("Index out of bounds in `ndx1`");
        }
    }
}

fn display(txt1: &str, txt2: &str) {
    let len = txt1.len();
    // Ensure `txt2` has the correct length
    debug_assert_eq!(len, txt2.len());
    let score = txt1.chars().zip(txt2.chars()).filter(|&(a, b)| a == b).count();
    println!("{}, {}, ({})", txt1, txt2, score);
}

fn main() {
    let data = ["abracadabra", "seesaw", "elk", "grrrrrr", "up", "a", "aabbbbaa", "", "xxxxx"];
    for &txt in &data {
        // Ensure `shuf` has the correct length
        let shuf_len = txt.len();
        let mut shuf = vec!['\0'; shuf_len];
        best_shuffle(txt, &mut shuf);
        display(txt, &shuf.iter().collect::<String>());
    }
    process::exit(0);
}