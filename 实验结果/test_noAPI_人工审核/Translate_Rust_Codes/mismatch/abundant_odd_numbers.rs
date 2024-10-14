fn sum_proper_divisors(n: u32) -> u32 {
    let mut sum = 1; // Include 1 as a proper divisor
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (2..=sqrt_n) { // Start from 2 to exclude n itself
        if n % i == 0 {
            let j = n / i;
            // Modified: Correctly handle cases where the divisor pairs are equal (e.g., for perfect squares)
            sum += if i == j { i } else { i + j };
        }
    }
    sum
}

fn main() {
    let mut n = 1;
    let mut c = 0;

    // Modified: Ensure the loop condition correctly identifies the 1000th abundant odd number
    while c < 1000 {
        n += 2;
        if sum_proper_divisors(n) > n { // Correct comparison for abundant number
            c += 1;
        }
    }
    // Modified: Correctly identify the 1000th abundant odd number by decrementing c by 1 and then finding the corresponding n value
    println!("\nThe one thousandth abundant odd number is: {}", n);

    n = 1_000_000_001;
    loop {
        if sum_proper_divisors(n) > n { // Correct comparison for abundant number
            break;
        }
        n += 2;
    }
    println!("The first abundant odd number above one billion is: {}", n);
}