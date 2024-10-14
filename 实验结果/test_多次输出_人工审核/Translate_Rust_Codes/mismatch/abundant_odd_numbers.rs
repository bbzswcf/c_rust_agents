fn sum_proper_divisors(n: u32) -> u32 {
    let mut sum = 1;
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
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

    while c < 25 {
        n += 2;
        if n < sum_proper_divisors(n) {
            c += 1;
            println!("{}: {}", c, n);
        }
    }

    while c < 1000 {
        n += 2;
        if n < sum_proper_divisors(n) {
            c += 1;
        }
    }
    println!("\nThe one thousandth abundant odd number is: {}", n);

    n = 1_000_000_001;
    loop {
        if n < sum_proper_divisors(n) {
            break;
        }
        n += 2;
    }
    println!("The first abundant odd number above one billion is: {}", n);
}