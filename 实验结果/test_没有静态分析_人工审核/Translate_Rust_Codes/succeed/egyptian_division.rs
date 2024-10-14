fn egyptian_division(dividend: u64, divisor: u64, remainder: Option<&mut u64>) -> u64 {
    let mut quotient = 0;
    let mut current_divisor = divisor;
    let mut current_power = 1;

    while current_divisor <= dividend {
        current_divisor <<= 1;
        current_power <<= 1;
    }

    current_divisor >>= 1;
    current_power >>= 1;

    let mut remaining_dividend = dividend;

    while current_power > 0 {
        if current_divisor <= remaining_dividend {
            remaining_dividend -= current_divisor;
            quotient += current_power;
        }
        current_divisor >>= 1;
        current_power >>= 1;
    }

    if let Some(rem) = remainder {
        *rem = remaining_dividend;
    }

    quotient
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