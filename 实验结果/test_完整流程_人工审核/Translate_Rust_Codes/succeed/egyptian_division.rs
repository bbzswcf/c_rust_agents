fn egyptian_division(dividend: u64, divisor: u64, remainder: Option<&mut u64>) -> u64 {
    let mut powers = [0u64; 64];
    let mut doublings = [0u64; 64];

    // Initialize powers and doublings, checking for overflow during shift
    for i in 0..64 {
        if divisor << i > u64::MAX / 2 {
            break; // Prevent overflow by breaking out of the loop early
        }
        powers[i] = 1 << i;
        doublings[i] = divisor << i;
    }

    let mut answer = 0;
    let mut accumulator = 0;

    // Iterate in reverse to build the answer and accumulator, checking for overflow during addition
    for i in (0..64).rev() {
        // Prevent subtraction overflow by ensuring doublings[i] is less than or equal to dividend
        if doublings[i] <= dividend && accumulator <= dividend - doublings[i] {
            accumulator += doublings[i];
            answer += powers[i];
        }
    }

    if let Some(rem) = remainder {
        *rem = dividend - accumulator; // Verified that accumulator is correctly accumulated
    }
    answer
}

fn go(a: u64, b: u64) {
    let mut y = 0;
    let x = egyptian_division(a, b, Some(&mut y));
    println!("{} / {} = {} remainder {}", a, b, x, y);
    assert!(a == b * x + y); // Verified that the assertion correctly checks the division and remainder
}

fn main() {
    go(580, 32);
}