fn swap(s: &mut [char], i: usize, j: usize) {
    let temp = s[i];
    s[i] = s[j];
    s[j] = temp;
}

fn reverse(s: &mut [char], mut i: usize, mut j: usize) {
    while i < j {
        swap(s, i, j);
        i += 1;
        j -= 1;
    }
}

fn next_permutation(s: &mut [char]) -> bool {
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
    let mut str = format!("{}", n).chars().collect::<Vec<char>>();
    if !next_permutation(&mut str) {
        return 0;
    }
    str.iter().collect::<String>().parse::<u32>().unwrap_or(0)
}

fn main() {
    let numbers: [u32; 8] = [0, 9, 12, 21, 12453, 738440, 45072010, 95322020];
    let count = numbers.len();
    for i in 0..count {
        println!("{} -> {}", numbers[i], next_highest_int(numbers[i]));
    }

    let big = "9589776899767587796600";
    let mut next = big.chars().collect::<Vec<char>>();
    next_permutation(&mut next);
    println!("{} -> {}", big, next.iter().collect::<String>());
}