fn egyptian_division(dividend: u64, divisor: u64, remainder: Option<&mut u64>) -> u64 {
    let mut powers = [0u64; 64];
    let mut doublings = [0u64; 64];

    let mut i = 0;
    for i in 0..64 {
        powers[i] = 1 << i;
        doublings[i] = divisor << i;
        if doublings[i] > dividend {
            break;
        }
    }

    let mut answer = 0;
    let mut accumulator = 0;

    // Modified: Adjusted the range to avoid underflow when `i` is 0
    for j in (0..i).rev() {
        if accumulator + doublings[j] <= dividend {
            accumulator += doublings[j];
            answer += powers[j];
        }
    }

    if let Some(rem) = remainder {
        *rem = dividend - accumulator;
    }
    answer
}

fn go(a: u64, b: u64) {
    let mut y = 0;
    let x = egyptian_division(a, b, Some(&mut y));
    println!("{} / {} = {} remainder {}", a, b, x, y);
    assert!(a == b * x + y);
}

fn main() {
    go(580, 32);
}