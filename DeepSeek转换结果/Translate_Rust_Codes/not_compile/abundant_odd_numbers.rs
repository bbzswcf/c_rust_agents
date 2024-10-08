fn sum_proper_divisors(n: u64) -> u64 {
    let mut sum = 1;
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            let j = n / i;
            sum += i;
            if i != j {
                sum += j;
            }
        }
    }
    sum
}

fn main() {
    let mut n = 1;
    let mut c = 0;

    // 1000th abundant odd number
    while c < 1000 {
        n += 2;
        if n < sum_proper_divisors(n) {
            c += 1;
        }
    }
    println!("\nThe one thousandth abundant odd number is: {}", n);

    // First abundant odd number above one billion
    n = 1000000001;
    loop {
        if n < sum_proper_divisors(n) {
            break;
        }
        n += 2;
    }
    println!("The first abundant odd number above one billion is: {}", n);
}