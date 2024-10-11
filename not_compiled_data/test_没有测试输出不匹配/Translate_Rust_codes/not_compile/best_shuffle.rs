use std::process;
use std::ptr;

fn best_shuffle(txt: &str, result: &mut [char]) {
    let len = txt.len();
    if len == 0 {
        return;
    }

    // txt and result must have the same length
    assert!(len == result.len());

    // how many of each character?
    let mut counts = vec![0; u8::MAX as usize + 1];
    let mut fmax = 0;
    for &ch in txt.as_bytes() {
        counts[ch as usize] += 1;
        if fmax < counts[ch as usize] {
            fmax = counts[ch as usize];
        }
    }
    assert!(fmax > 0 && fmax <= len);

    // all character positions, grouped by character
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

    // regroup them for cycles
    let mut ndx2 = vec![0; len];
    let mut n = 0;
    let mut m = 0;
    for i in 0..len {
        ndx2[i] = ndx1[n];
        n += fmax;
        if n >= len {
            m += 1;
            n = m;
        }
    }

    // how long can our cyclic groups be?
    let grp = 1 + (len - 1) / fmax;
    assert!(grp > 0 && grp <= len);

    // how many of them are full length?
    let lng = 1 + (len - 1) % fmax;
    assert!(lng > 0 && lng <= len);

    // rotate each group
    let mut j = 0;
    for i in 0..fmax {
        let first = ndx2[j];
        let glen = grp - (i < lng) as usize;
        for k in 1..glen {
            ndx1[j + k - 1] = ndx2[j + k];
        }
        ndx1[j + glen - 1] = first;
        j += glen;
    }

    // result is original permuted according to our cyclic groups
    result[len] = '\0';
    for i in 0..len {
        result[ndx2[i]] = txt.chars().nth(ndx1[i]).unwrap();
    }
}

fn display(txt1: &str, txt2: &str) {
    let len = txt1.len();
    assert!(len == txt2.len());
    let score = txt1.chars().zip(txt2.chars()).filter(|&(a, b)| a == b).count();
    println!("{}, {}, ({})", txt1, txt2, score);
}

fn main() {
    let data = [
        "abracadabra", "seesaw", "elk", "grrrrrr", "up", "a", "aabbbbaa", "", "xxxxx",
    ];
    for &txt in &data {
        let shuf_len = txt.len() + 1;
        let mut shuf = vec!['\0'; shuf_len];
        best_shuffle(txt, &mut shuf);
        display(txt, &shuf.iter().collect::<String>());
    }
}