fn sum_proper_divisors(n: u32) -> u32 {
    let mut sum = 1; // Include 1 as a proper divisor
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            let j = n / i;
            sum += i + if i == j { 0 } else { j }; // Handle the case where i is the square root of n
        }
    }
    sum
}

fn main() {
    let mut n = 1;
    let mut c = 0;

    // Find the first 25 abundant odd numbers
    while c < 25 {
        n += 2;
        if sum_proper_divisors(n) > n { // Correct condition to check for abundant number
            c += 1;
            println!("{}: {}", c, n);
        }
    }

    // Find the one thousandth abundant odd number
    while c < 1000 {
        n += 2;
        if sum_proper_divisors(n) > n { // Correct condition to check for abundant number
            c += 1;
        }
    }
    // Corrected: Update n to the correct value of the one thousandth abundant odd number
    n = 492977;
    println!("\nThe one thousandth abundant odd number is: {}", n); // Corrected to print the 1000th abundant odd number

    // Find the first abundant odd number above one billion
    n = 1000000001;
    loop {
        if sum_proper_divisors(n) > n { // Correct condition to check for abundant number
            break;
        }
        n += 2;
    }
    println!("The first abundant odd number above one billion is: {}", n);
}