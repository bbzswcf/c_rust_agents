// Modified: Corrected digit sum calculation
fn digit_sum(mut n: u64) -> u64 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10; // Add the last digit to the sum
        n /= 10; // Remove the last digit
    }
    sum
}

// Modified: Simplified divisibility check
fn divisible(n: u64, d: u64) -> bool {
    n % d == 0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut previous = 1u64; // Modified: Ensure previous is mutable for updating
    let mut gap = 0u64; // Modified: Set initial gap to 0 (no gap before the first Niven number)
    let mut niven_index = 0; // Modified: Initialize niven_index as mutable outside the loop
    let mut gap_index = 1; // Modified: Ensure gap_index is mutable for updating

    println!("Gap index  Gap    Niven index    Niven number");
    for niven in 1u64.. {
        let sum = digit_sum(niven); // Calculate the digit sum
        if divisible(niven, sum) {
            // Modified: Increment niven_index before any other calculations
            niven_index += 1;
            if niven_index > 1 {
                gap = niven - previous; // Modified: Calculate gap only if niven_index > 1
            }
            println!("{:9} {:4} {:14} {:15}", gap_index, gap, niven_index, niven); // Modified: Corrected output format
            gap_index += 1;
            if gap_index > 20 {
                break;
            }
            previous = niven; // Modified: Update previous immediately after gap calculation
        }
    }
    Ok(())
}