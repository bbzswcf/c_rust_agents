fn sum_proper_divisors(n: u32) -> u32 {
    let mut sum = 1; // Initialize sum to 1 to include 1 as a proper divisor for all numbers greater than 1
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (2..=sqrt_n) { // Start from 2
        if n % i == 0 {
            let j = n / i;
            sum += i + if i == j { 0 } else { j };
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
        if sum_proper_divisors(n) > n { // Correct comparison to find abundant numbers
            c += 1;
            println!("{}: {}", c, n);
        }
    }

    // Find the one thousandth abundant odd number
    n = 1; // Reset n to 1 to start counting from the beginning
    c = 0; // Reset c to 0 to start counting from the beginning
    let mut thousandth_abundant_odd = 0; // Variable to store the 1000th abundant odd number
    while c < 1000 {
        n += 2;
        if sum_proper_divisors(n) > n { // Correct comparison to find abundant numbers
            c += 1;
            if c == 1000 {
                thousandth_abundant_odd = n; // Store the 1000th abundant odd number
            }
        }
    }
    println!("\nThe one thousandth abundant odd number is: {}", thousandth_abundant_odd); // Print the 1000th number

    // Find the first abundant odd number above one billion
    n = 1000000001; // Correct initial value for finding the first abundant odd number above one billion
    loop {
        if sum_proper_divisors(n) > n { // Correct comparison to find abundant numbers
            break;
        }
        n += 2; // Increment by 2 to ensure n remains odd
    }
    println!("The first abundant odd number above one billion is: {}", n);
}