use std::convert::TryInto; // Added: Import `TryInto` trait to use `try_into()` method

type ULL = u64;

fn is89(mut x: i32) -> bool {
    loop {
        let mut s = 0;
        while x > 0 {
            let digit = x % 10;
            s += digit * digit;
            x /= 10;
        }

        if s == 89 {
            return true;
        }
        if s == 1 {
            return false;
        }
        x = s;
    }
}

fn main() {
    let mut sums = vec![0; 32 * 81 + 1];
    sums[0] = 1;

    for n in 1.. {
        for i in (1..=n * 81).rev() {
            for j in 1..10 {
                let s = j * j;
                if s > i {
                    break;
                }
                sums[i] += sums[i - s];
            }
        }

        // Modified: Use `u64` for `count89` to handle larger numbers
        let mut count89: u64 = 0;
        for i in 1..n * 81 + 1 {
            // Modified: Directly use `i` without conversion since `i` is within `usize` bounds
            if !is89(i as i32) {
                continue;
            }

            // Modified: Use `checked_add` to handle potential overflow more safely
            if let Some(new_count) = count89.checked_add(sums[i]) {
                count89 = new_count;
            } else {
                println!("counter overflow for 10^{}", n);
                break; // Modified: Continue the loop after printing the overflow message
            }
        }

        println!("1->10^{}: {}", n, count89);
    }
}