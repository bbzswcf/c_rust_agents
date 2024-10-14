use std::ptr;

fn swap(str: &mut [char], i: usize, j: usize) {
    let c = str[i];
    str[i] = str[j];
    str[j] = c;
}

fn reverse(str: &mut [char], i: usize, j: usize) {
    let (mut i, mut j) = (i, j);
    while i < j {
        swap(str, i, j);
        i += 1;
        j -= 1;
    }
}

fn next_permutation(str: &mut [char]) -> bool {
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
    let str: Vec<char> = format!("{}", n).chars().collect();
    let mut str_chars: Vec<char> = str.iter().map(|&c| c).collect();
    if !next_permutation(&mut str_chars) {
        return 0;
    }
    str_chars.iter().collect::<String>().parse::<u32>().unwrap_or(0)
}

fn main() {
    let numbers: [u32; 8] = [0, 9, 12, 21, 12453, 738440, 45072010, 95322020];
    for &num in numbers.iter() {
        println!("{} -> {}", num, next_highest_int(num));
    }

    let big = "9589776899767587796600";
    let mut next: Vec<char> = big.chars().collect();
    next_permutation(&mut next);
    println!("{} -> {}", big, next.iter().collect::<String>());
}