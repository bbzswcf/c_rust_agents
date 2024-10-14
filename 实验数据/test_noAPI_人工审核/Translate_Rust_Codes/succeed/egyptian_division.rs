use std::io;

fn egyptian_division(dividend: u64, divisor: u64, remainder: Option<&mut u64>) -> u64 {
    let mut powers = [0u64; 64];
    let mut doublings = [0u64; 64];

    // Modified: Ensure the arrays are correctly initialized within the loop
    for i in 0..64 {
        powers[i] = 1 << i;
        // Modified: Correct calculation of `doublings` array
        if let Some(doubling) = divisor.checked_mul(1 << i) {
            doublings[i] = doubling;
        } else {
            // Break out of the loop early if overflow is detected
            break;
        }
    }

    let mut answer = 0;
    let mut accumulator = 0;

    // Modified: Correctly iterate over the initialized part of the arrays
    let last_initialized_index = doublings.iter().position(|&x| x == 0).unwrap_or(64) - 1;
    for i in (0..=last_initialized_index).rev() {
        if accumulator + doublings[i] <= dividend {
            accumulator += doublings[i];
            answer += powers[i];
        }
    }

    // Modified: Ensure the `remainder` is correctly calculated and assigned
    if let Some(rem) = remainder {
        *rem = dividend - accumulator;
    }
    answer
}

fn go(a: u64, b: u64) {
    let mut y = 0;
    let x = egyptian_division(a, b, Some(&mut y));
    println!("{} / {} = {} remainder {}", a, b, x, y);
    // Modified: Ensure the assertion correctly checks the division and remainder
    assert_eq!(a, b * x + y);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    go(580, 32);
    Ok(())
}