use std::mem;
use std::process;

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
    let mut ndx1 = Vec::with_capacity(len);
    for ch in 0..=u8::MAX {
        if counts[ch as usize] > 0 {
            for (j, &txt_ch) in txt.as_bytes().iter().enumerate() {
                if ch == txt_ch {
                    ndx1.push(j);
                }
            }
        }
    }

    // regroup them for cycles
    let mut ndx2 = Vec::with_capacity(len);
    for i in 0..len {
        ndx2.push(ndx1[i]);
    }
    for i in 0..len {
        let n = i * fmax;
        let m = i / fmax;
        ndx2[i] = ndx1[n % len + m];
    }

    // how long can our cyclic groups be?
    let grp = 1 + (len - 1) / fmax;
    assert!(grp > 0 && grp <= len);

    // how many of them are full length?
    let lng = 1 + (len - 1) % fmax;
    assert!(lng > 0 && lng <= len);

    // rotate each group
    for i in 0..fmax {
        let first = ndx2[i * grp];
        let glen = grp - (i < lng) as usize;
        for k in 1..glen {
            ndx1[i * grp + k - 1] = ndx2[i * grp + k];
        }
        ndx1[i * grp + glen - 1] = first;
    }

    // result is original permuted according to our cyclic groups
    for i in 0..len {
        // 修改: 检查索引是否在有效范围内，避免panic
        if ndx2[i] < len {
            result[ndx2[i]] = txt.chars().nth(ndx1[i]).unwrap_or('\0');
        }
    }
}

fn display(txt1: &str, txt2: &str) {
    let len = txt1.len();
    if len == 0 {
        return;
    }
    assert!(len == txt2.len());
    let score = txt1.chars().zip(txt2.chars()).filter(|&(a, b)| a == b).count();
    print!("{}, {}, ({})\n", txt1, txt2, score);
}

fn main() {
    let data = ["abracadabra", "seesaw", "elk", "grrrrrr", "up", "a", "aabbbbaa", "", "xxxxx"];
    for &txt in &data {
        let shuf_len = txt.len();
        // 修改: 检查shuf_len是否为0，避免创建空向量
        if shuf_len == 0 {
            continue;
        }
        let mut shuf = vec!['\0'; shuf_len];
        best_shuffle(txt, &mut shuf);
        let shuf_str = shuf.iter().filter(|&&c| c != '\0').collect::<String>();
        display(txt, &shuf_str);
    }
}