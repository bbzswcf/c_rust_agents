use std::ptr;

fn swap(str: &mut [u8], i: usize, j: usize) {
    let c = str[i];
    str[i] = str[j];
    str[j] = c;
}

fn reverse(str: &mut [u8], i: usize, j: usize) {
    let mut i = i;
    let mut j = j;
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
    let str = format!("{}", n);
    let mut str_bytes = str.into_bytes();
    if !next_permutation(&mut str_bytes) {
        return 0;
    }
    let next_str = String::from_utf8(str_bytes).unwrap();
    next_str.parse::<u32>().unwrap()
}

fn main() {
    let numbers: [u32; 8] = [0, 9, 12, 21, 12453, 738440, 45072010, 95322020];
    for &num in numbers.iter() {
        println!("{} -> {}", num, next_highest_int(num));
    }

    let big = "9589776899767587796600";
    let mut next = [0u8; 22];
    unsafe {
        ptr::copy_nonoverlapping(big.as_ptr(), next.as_mut_ptr(), big.len());
    }
    next_permutation(&mut next);
    let next_str = String::from_utf8_lossy(&next);
    println!("{} -> {}", big, next_str);
}