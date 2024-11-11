use std::mem;
use std::process;

fn best_shuffle(txt: &str, result: &mut [char]) {
    let len = txt.len();
    if len == 0 {
        return;
    }

    debug_assert!(len == result.len()); // Adjust assertion to account for no null terminator

    let mut counts = vec![0; u8::MAX as usize + 1];
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
        ndx2[i] = ndx1[n];
        n += fmax;
        if n >= len {
            m += 1;
            n = m;
        }
        // Ensure ndx2[i] is within bounds
        if ndx2[i] >= len {
            ndx2[i] = len - 1;
        }
        // Ensure ndx1[n] is within bounds
        if n >= len {
            n = m;
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
            if i * grp + k - 1 < len {
                ndx1[i * grp + k - 1] = ndx2[i * grp + k];
            }
        }
        if i * grp + glen - 1 < len {
            ndx1[i * grp + glen - 1] = first;
        }
    }

    // Remove the null terminator assignment

    for i in 0..len {
        let index = ndx2[i];
        if index < len && ndx1[i] < len {
            result[index] = txt.chars().nth(ndx1[i]).unwrap_or('\0');
        }
    }
}

fn display(txt1: &str, txt2: &[char]) {
    let len = txt1.len();
    debug_assert!(len == txt2.len());
    let mut score = 0;
    for (c1, &c2) in txt1.chars().zip(txt2.iter()) {
        if c1 == c2 {
            score += 1;
        }
    }
    let txt2_str: String = txt2.iter().collect::<String>();
    println!("{}, {}, ({})", txt1, txt2_str, score);
}

fn main() {
    let data = ["abracadabra", "seesaw", "elk", "grrrrrr", "up", "a", "aabbbbaa", "", "xxxxx"];
    for &txt in &data {
        let mut shuf = vec!['\0'; txt.len()]; // Adjusted size to exclude null terminator
        best_shuffle(txt, &mut shuf);
        display(txt, &shuf); // Pass the correct slice to display
    }
}