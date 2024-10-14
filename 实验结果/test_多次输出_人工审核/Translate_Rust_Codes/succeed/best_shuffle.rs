use std::collections::HashMap;
use std::mem;

fn best_shuffle(txt: &str, result: &mut [char]) {
    let len = txt.len();
    if len == 0 {
        return;
    }

    // txt and result must have the same length
    debug_assert_eq!(len, result.len());

    // how many of each character?
    let mut counts = vec![0; u8::MAX as usize + 1];
    let mut fmax = 0;
    for ch in txt.chars() {
        let ch_idx = ch as usize;
        counts[ch_idx] += 1;
        if fmax < counts[ch_idx] {
            fmax = counts[ch_idx];
        }
    }
    debug_assert!(fmax > 0 && fmax <= len);

    // all character positions, grouped by character
    let mut ndx1 = vec![0; len];
    let mut idx_map = HashMap::new();
    for (i, ch) in txt.char_indices() {
        idx_map.entry(ch).or_insert_with(Vec::new).push(i);
    }
    let mut i = 0;
    for ch in 0..=u8::MAX {
        if let Some(positions) = idx_map.get(&(ch as char)) {
            for &pos in positions {
                ndx1[i] = pos;
                i += 1;
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
    debug_assert!(grp > 0 && grp <= len);

    // how many of them are full length?
    let lng = 1 + (len - 1) % fmax;
    debug_assert!(lng > 0 && lng <= len);

    // rotate each group
    let mut j = 0;
    for i in 0..fmax {
        let first = ndx2[j];
        let glen = grp - if i < lng { 0 } else { 1 };
        for k in 1..glen {
            ndx1[j + k - 1] = ndx2[j + k];
        }
        ndx1[j + glen - 1] = first;
        j += glen;
    }

    // result is original permuted according to our cyclic groups
    for i in 0..len {
        result[ndx2[i]] = txt.chars().nth(ndx1[i]).unwrap();
    }
}

fn display(txt1: &str, txt2: &str) {
    let len = txt1.len();
    debug_assert_eq!(len, txt2.len());
    let score = txt1.chars().zip(txt2.chars()).filter(|&(a, b)| a == b).count();
    println!("{}, {}, ({})", txt1, txt2, score);
}

fn main() {
    let data = ["abracadabra", "seesaw", "elk", "grrrrrr", "up", "a", "aabbbbaa", "", "xxxxx"];
    for &txt in &data {
        let len = txt.len();
        let mut shuf = vec!['\0'; len];
        best_shuffle(txt, &mut shuf);
        display(txt, &shuf.iter().collect::<String>());
    }
}