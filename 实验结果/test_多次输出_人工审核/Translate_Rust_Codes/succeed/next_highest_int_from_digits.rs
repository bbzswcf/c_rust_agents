fn swap(s: &mut [u8], i: usize, j: usize) {
    let temp = s[i];
    s[i] = s[j];
    s[j] = temp;
}

fn reverse(s: &mut [u8], i: usize, j: usize) {
    let (mut i, mut j) = (i, j);
    while i < j {
        swap(s, i, j);
        i += 1;
        j -= 1;
    }
}

fn next_permutation(s: &mut [u8]) -> bool {
    let len = s.len();
    if len < 2 {
        return false;
    }
    for i in (1..len).rev() {
        let j = i;
        if s[i - 1] < s[j] {
            let mut k = len;
            while s[i - 1] >= s[k - 1] {
                k -= 1;
            }
            swap(s, i - 1, k - 1);
            reverse(s, j, len - 1);
            return true;
        }
    }
    false
}

fn next_highest_int(n: u32) -> u32 {
    let mut str = format!("{}", n).into_bytes();
    if !next_permutation(&mut str) {
        return 0;
    }
    str.iter().fold(0, |acc, &digit| acc * 10 + (digit - b'0') as u32)
}

fn main() {
    let numbers = [0, 9, 12, 21, 12453, 738440, 45072010, 95322020];
    for &num in &numbers {
        println!("{} -> {}", num, next_highest_int(num));
    }

    let big = "9589776899767587796600";
    let mut next = big.to_string().into_bytes();
    next_permutation(&mut next);
    println!("{} -> {}", big, String::from_utf8(next).unwrap());
}