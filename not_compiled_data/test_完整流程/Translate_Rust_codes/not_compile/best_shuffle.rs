use std::process;
use std::ptr;

fn best_shuffle(txt: &str, result: &mut [char]) {
    let len = txt.len();
    if len == 0 {
        return;
    }

    // Ensure `result` has the same length as `txt`
    assert_eq!(len, result.len());

    // how many of each character?
    let mut counts = vec![0; u8::MAX as usize + 1];
    let mut fmax = 0;
    for &ch in txt.as_bytes() {
        counts[ch as usize] += 1;
        if fmax < counts[ch as usize] {
            fmax = counts[ch as usize];
        }
    }
    // Allow `fmax` to be equal to `len`
    assert!(fmax > 0 && fmax <= len);

    // all character positions, grouped by character
    let mut ndx1 = Vec::with_capacity(len); // Modified: Pre-allocate with correct capacity
    for ch in 0..=u8::MAX {
        if counts[ch as usize] > 0 {
            for (j, &tch) in txt.as_bytes().iter().enumerate() {
                if ch == tch {
                    ndx1.push(j);
                }
            }
        }
    }

    // regroup them for cycles
    let mut ndx2 = vec![0; len]; // Modified: Pre-allocate with correct length
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

    // Ensure the assignment loop only iterates up to the original length of `txt`
    for i in 0..len {
        // Ensure `ndx2[i]` and `ndx1[i]` are within valid bounds before accessing `result` and `txt` respectively
        if ndx2[i] < len && ndx1[i] < txt.len() {
            result[ndx2[i]] = txt.chars().nth(ndx1[i]).unwrap();
        }
    }
}

fn display(txt1: &str, txt2: &str) {
    let len = txt1.len();
    assert_eq!(len, txt2.len()); // Ensure `txt1` and `txt2` have the same length
    let score = txt1.chars().zip(txt2.chars()).filter(|&(a, b)| a == b).count();
    println!("{}, {}, ({})", txt1, txt2, score);
}

fn main() {
    let data = [
        "abracadabra", "seesaw", "elk", "grrrrrr", "up", "a", "aabbbbaa", "", "xxxxx",
    ];
    for &txt in &data {
        let shuf_len = txt.len(); // Correct the length calculation for `shuf`
        let mut shuf = vec!['\0'; shuf_len];
        best_shuffle(txt, &mut shuf);
        // Include all characters up to the length of `txt`
        display(txt, &shuf[..txt.len()].iter().collect::<String>());
    }
}