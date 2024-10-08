use std::mem;

fn swap(str: &mut [u8], i: usize, j: usize) {
    let c = str[i];
    str[i] = str[j];
    str[j] = c;
}

fn reverse(str: &mut [u8], i: usize, j: usize) {
    let (mut i, mut j) = (i, j);
    while i < j {
        swap(str, i, j);
        i += 1;
        j -= 1;
    }
}

fn next_permutation(str: &mut [u8]) -> bool {
    let len = str.len();
    if len < 2 {
        return false;
    }
    for i in (1..len).rev() {
        let j = i;
        let i = i - 1;
        if str[i] < str[j] {
            let mut k = len;
            while str[i] >= str[k - 1] {
                k -= 1;
            }
            swap(str, i, k - 1);
            reverse(str, j, len - 1);
            return true;
        }
    }
    false
}

fn next_highest_int(n: u32) -> u32 {
    let mut str = [0u8; 16];
    let len = n.to_string().len();
    let s = n.to_string();
    str[..len].copy_from_slice(s.as_bytes());
    if !next_permutation(&mut str[..len]) {
        return 0;
    }
    str[..len].iter().map(|&c| c as char).collect::<String>().parse().unwrap_or(0)
}

fn main() {
    let numbers: [u32; 8] = [0, 9, 12, 21, 12453, 738440, 45072010, 95322020];
    for &num in &numbers {
        println!("{} -> {}", num, next_highest_int(num));
    }

    let big = "9589776899767587796600";
    let mut next = big.as_bytes().to_vec();
    next_permutation(&mut next);
    let next_str = String::from_utf8(next).unwrap();
    println!("{} -> {}", big, next_str);
}