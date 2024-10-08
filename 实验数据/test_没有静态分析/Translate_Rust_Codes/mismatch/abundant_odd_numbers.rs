fn sum_proper_divisors(n: u32) -> u32 {
    let mut sum: u64 = 0; // Modified: Initialize sum to 0 instead of 1
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in 1..=sqrt_n {
        if n % i == 0 {
            let j = n / i;
            if i == j {
                sum += i as u64; // Modified: Only add i once if i == j
            } else if i != 1 {
                sum += (i + j) as u64; // Modified: Add both i and j if i != 1
            } else {
                sum += i as u64; // Modified: Add i if i == 1
            }
        }
    }
    sum as u32
}

fn main() {
    let mut n = 1;
    let mut c = 0;

    while c < 1000 {
        n += 2;
        if sum_proper_divisors(n) > n {
            c += 1;
        }
    }
    println!("The one thousandth abundant odd number is: {}", n); // Modified: Removed extra newline character

    n = 1000000001;
    loop {
        if sum_proper_divisors(n) > n {
            break;
        }
        n += 2;
    }
    println!("The first abundant odd number above one billion is: {}", n);
}