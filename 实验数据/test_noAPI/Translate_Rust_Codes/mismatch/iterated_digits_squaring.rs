use std::io;

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
    // Modified: Changed the type of `sums` elements to `u64` during initialization
    let mut sums = vec![1u64, 0u64];
    sums.resize(32 * 81 + 1, 0u64);

    for n in 1.. { // Infinite loop, ensure there is a condition to break the loop
        for i in (1..=n * 81).rev() {
            for j in 1..10 {
                let s = j * j;
                if s > i {
                    break;
                }
                sums[i] += sums[i - s];
            }
        }

        // Modified: Ensure `count89` is of type `u64` to match the type of `sums[i]`
        let mut count89: u64 = 0;
        for i in 1..=n * 81 {
            // Modified: Cast `i` to `i32` before passing it to `is89`
            if !is89(i as i32) {
                continue;
            }

            // Modified: Replace `!0u64` with `u64::MAX` for clarity and correctness
            if sums[i] > u64::MAX - count89 {
                println!("counter overflow for 10^{}", n);
                return;
            }
            count89 += sums[i];
        }

        println!("1->10^{}: {}", n, count89);

        // Add a condition to break the loop if needed
        if n == 10 {
            break;
        }
    }
}