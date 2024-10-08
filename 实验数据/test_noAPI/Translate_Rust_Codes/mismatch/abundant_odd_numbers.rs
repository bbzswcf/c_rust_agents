fn sum_proper_divisors(n: u64) -> u64 {
    let mut sum = 1; // Ensure 1 is included as a proper divisor
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            let j = n / i;
            sum += i;
            if i != j {
                sum += j; // Avoid double-counting the square root
            }
        }
    }
    sum
}

fn main() {
    let mut n = 1;
    let mut c = 0;

    while c < 25 {
        n += 2;
        if sum_proper_divisors(n) > n {
            c += 1;
            println!("{}: {}", c, n);
        }
    }

    // Reset n and c for the next loop
    n = 1;
    c = 0;

    // Modified: Ensure the loop correctly counts up to 1000 without skipping any numbers
    while c < 1000 {
        n += 2;
        if sum_proper_divisors(n) > n {
            c += 1;
            if c == 1000 {
                println!("\nThe one thousandth abundant odd number is: {}", n); // Ensure correct counting and identification
                break;
            }
        }
    }

    n = 1000000001; // Ensure n starts as an odd number above one billion
    loop {
        if sum_proper_divisors(n) > n {
            println!("The first abundant odd number above one billion is: {}", n);
            break;
        }
        n += 2;
        // Modified: Added early exit condition to avoid infinite loop
        if n > 10000000000 {
            println!("No abundant odd number found within reasonable range.");
            break;
        }
    }
}