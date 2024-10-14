use std::fmt;
use std::ptr;

// Modified: Changed the type of `str` to `&mut [u8]` to match the usage in `next_highest_int` and `main`.
fn swap(str: &mut [u8], i: usize, j: usize) {
    let c = str[i];
    str[i] = str[j];
    str[j] = c;
}

// Modified: Changed the type of `str` to `&mut [u8]` to match the usage in `next_highest_int` and `main`.
fn reverse(str: &mut [u8], i: usize, j: usize) {
    let (mut i, mut j) = (i, j);
    while i < j {
        swap(str, i, j);
        i += 1;
        j -= 1;
    }
}

// Modified: Changed the type of `str` to `&mut [u8]` to match the usage in `next_highest_int` and `main`.
fn next_permutation(str: &mut [u8]) -> bool {
    let len = str.len();
    if len < 2 {
        return false;
    }
    for i in (1..len).rev() {
        let j = i;
        if str[i - 1] < str[j] {
            let mut k = len;
            while str[i - 1] >= str[k - 1] {
                k -= 1;
            }
            swap(str, i - 1, k - 1);
            reverse(str, j, len - 1);
            return true;
        }
    }
    false
}

fn next_highest_int(n: u32) -> u32 {
    let mut str = [0u8; 16];
    let s = format!("{}", n);
    for (i, c) in s.chars().enumerate() {
        str[i] = c as u8; // Modified: Ensure that the type of `c` is correctly cast to `u8`.
    }
    if !next_permutation(&mut str[..s.len()]) {
        return 0;
    }
    let next_str: String = str.iter().take(s.len()).map(|&c| c as char).collect(); // Modified: Ensure that the type of `c` is correctly cast to `char`.
    next_str.parse::<u32>().unwrap_or(0)
}

fn main() {
    let numbers: [u32; 8] = [0, 9, 12, 21, 12453, 738440, 45072010, 95322020];
    for &num in &numbers {
        println!("{} -> {}", num, next_highest_int(num));
    }

    let big = "9589776899767587796600";
    let mut next = [0u8; 22]; // Modified: Ensure that the type of `next` is consistent with the expected type.
    for (i, &c) in big.as_bytes().iter().enumerate() {
        next[i] = c; // Modified: Ensure that the type of `c` is consistent with the expected type.
    }
    next_permutation(&mut next); // Modified: Ensure that the type of `next` is consistent with the expected type.
    let next_str: String = next.iter().map(|&c| c as char).collect(); // Modified: Ensure that the type of `c` is correctly cast to `char`.
    println!("{} -> {}", big, next_str);
}