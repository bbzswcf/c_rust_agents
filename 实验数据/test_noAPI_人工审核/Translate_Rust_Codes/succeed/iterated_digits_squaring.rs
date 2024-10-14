use std::io;
use std::convert::TryInto; // Added: Import the TryInto trait to use try_into() method

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

fn main() -> io::Result<()> {
    let mut sums = vec![0_u64; 32 * 81 + 1];
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

        let mut count89 = 0_u64;
        for i in 1..=n * 81 {
            // Modified: Handle potential conversion error using match
            if let Ok(i32_value) = i.try_into() {
                if !is89(i32_value) {
                    continue;
                }
            } else {
                // Handle the conversion error, e.g., by logging or returning an error
                eprintln!("Conversion from usize to i32 failed");
                return Ok(());
            }

            // Modified: Use checked arithmetic to handle potential overflow safely
            if let Some(new_count) = count89.checked_add(sums[i]) {
                count89 = new_count;
            } else {
                println!("counter overflow for 10^{}", n);
                return Ok(());
            }
        }

        println!("1->10^{}: {}", n, count89);
    }

    Ok(())
}