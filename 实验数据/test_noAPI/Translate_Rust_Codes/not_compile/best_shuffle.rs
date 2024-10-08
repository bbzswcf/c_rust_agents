fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let result = plus_one(5);
    println!("Result: {}", result);

    if true {
        println!("This is a boolean condition");
    }

    let x: f32 = 3.14;
    println!("x: {}", x);

    let txt = "example";
    let mut result = Vec::with_capacity(txt.len()); // Modified: Removed unnecessary initialization
    let ndx1 = vec![0, 1, 2, 3, 4, 5, 6];
    let ndx2 = vec![0, 1, 2, 3, 4, 5, 6];

    for i in 0..ndx1.len().saturating_sub(1) { // Modified: Prevent accessing an index that is out of bounds
        if ndx1[i] < txt.len() {
            if let Some(ch) = txt.chars().nth(ndx1[i]) {
                if ndx2[i] < result.len() {
                    result.push(ch); // Modified: Directly populate the vector
                }
            }
        }
    }

    let data = ["abracadabra", "seesaw", "elk", "grrrrrr", "up", "a", "aabbbbaa", "", "xxxxx"];
    for &txt in &data {
        let shuf_len = txt.len();
        let mut shuf = Vec::with_capacity(shuf_len); // Modified: Removed unnecessary initialization

        best_shuffle(txt, &mut shuf);
        display(txt, &shuf.iter().collect::<String>());
    }
}

fn best_shuffle(txt: &str, result: &mut Vec<char>) {
    let len = txt.len();
    if len == 0 {
        return;
    }

    #[cfg(debug_assertions)]
    {
        assert_eq!(len, result.len());
    }

    let mut counts: [usize; u8::MAX as usize + 1] = [0; u8::MAX as usize + 1];
    let mut fmax = 0;
    for &ch in txt.as_bytes() {
        counts[ch as usize] += 1;
        if fmax < counts[ch as usize] {
            fmax = counts[ch as usize];
        }
    }

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
    }

    let grp = 1 + (len - 1) / fmax;
    let lng = 1 + (len - 1) % fmax;

    for i in 0..fmax {
        let first = ndx2[i * grp];
        let glen = grp - (i < lng) as usize;
        for k in 1..glen {
            if i * grp + k - 1 < ndx1.len() { // Modified: Ensure the index does not exceed bounds
                ndx1[i * grp + k - 1] = ndx2[i * grp + k];
            }
        }
        if i * grp + glen - 1 < ndx1.len() { // Modified: Ensure the index does not exceed bounds
            ndx1[i * grp + glen - 1] = first;
        }
    }

    for i in 0..len.saturating_sub(1) { // Modified: Prevent accessing an index that is out of bounds
        if ndx1[i] < txt.len() {
            if let Some(ch) = txt.chars().nth(ndx1[i]) {
                if ndx2[i] < result.len() {
                    result.push(ch); // Modified: Directly populate the vector
                }
            }
        }
    }
}

fn display(txt1: &str, txt2: &str) {
    let len = txt1.len();
    // Modified: Removed redundant length check
    let score = txt1.chars().zip(txt2.chars()).filter(|&(a, b)| a == b).count();
    println!("{}, {}, ({})", txt1, txt2, score);
}