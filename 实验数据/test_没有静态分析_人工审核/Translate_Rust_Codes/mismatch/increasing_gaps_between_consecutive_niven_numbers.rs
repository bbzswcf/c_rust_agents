use std::env;

// Function to calculate the sum of the digits of n.
fn digit_sum(n: u64) -> u64 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

// Function to check if n is divisible by d.
fn divisible(n: u64, d: u64) -> bool {
    n % d == 0
}

fn main() {
    // Removed: Unnecessary environment variable setting.

    let mut previous = 1; // Initialized to the first Niven number.
    let mut gap = 1; // Corrected: Initial gap value should be 1.
    let mut niven_index = 0;
    let mut gap_index = 1;

    println!("Gap index  Gap    Niven index    Niven number");
    let mut niven = 1;
    while gap_index <= 20 {
        let sum = digit_sum(niven);
        if divisible(niven, sum) {
            if niven > previous { // Corrected: Changed condition to correctly identify gaps.
                gap = niven - previous; // Corrected: Calculate gap only when a new Niven number is found.
                println!("{:9} {:4} {:14} {}", gap_index, gap, niven_index, niven);
                gap_index += 1;
            }
            previous = niven;
            niven_index += 1; // Corrected: Ensure niven_index is incremented correctly.
        }
        niven += 1; // Ensure niven is incremented correctly.
    }
}