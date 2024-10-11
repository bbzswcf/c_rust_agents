use std::io;

type ULL = u64;

fn is89(mut x: u64) -> bool { // Modified: Changed the type of `x` to `u64` to match the type of `i`
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
    // Modified: Change the type of `sums` to `Vec<u64>` to match the type of `count89`
    let mut sums: Vec<u64> = vec![1, 0];
    sums.resize(32 * 81 + 1, 0);

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

        let mut count89: u64 = 0; // Modified: Changed the type of `count89` to `u64` to match the type of `sums[i]`
        for i in 1..n * 81 + 1 {
            if !is89(i as u64) { // Modified: Changed the type of `i` to `u64` to match the type of `is89`
                continue;
            }

            // Modified: Ensure `sums[i]` is compared with a value of the same type
            if sums[i] > std::u64::MAX - count89 {
                // Modified: Ensure `n` is correctly formatted for printing
                println!("counter overflow for 10^{}", n);
                return;
            }
            // Modified: Ensure `sums[i]` is added to `count89` with the same type
            count89 += sums[i];
        }

        // Modified: Ensure `n` is correctly formatted for printing
        println!("1->10^{}: {}", n, count89);
    }
}