fn sieve(limit: usize, primes: &mut [usize], count: &mut usize) {
    let mut c = vec![false; limit + 1];
    let mut p = 3;
    let mut n = 0;
    while p * p <= limit {
        for i in (p * p..=limit).step_by(2 * p) {
            c[i] = true;
        }
        p += 2;
        while c[p] {
            p += 2;
        }
    }
    for i in (3..=limit).step_by(2) {
        if !c[i] {
            primes[n] = i;
            n += 1;
        }
    }
    *count = n;
}

fn find_period(n: usize) -> usize {
    let mut r = 1;
    let mut period = 0;
    for _ in 1..=n + 1 {
        r = (10 * r) % n;
    }
    let rr = r;
    loop {
        r = (10 * r) % n;
        period += 1;
        if r == rr {
            break;
        }
    }
    period
}

fn main() {
    let numbers = [500, 1000, 2000, 4000, 8000, 16000, 32000];
    let mut primes = vec![0; 6500];
    let mut prime_count = 0;
    let mut long_primes = vec![0; prime_count];
    let mut totals = vec![0; numbers.len()];
    let mut long_count = 0;
    let mut count = 0;
    let mut index = 0;

    sieve(32000, &mut primes, &mut prime_count);
    long_primes.resize(prime_count, 0);

    for &prime in &primes[..prime_count] {
        if find_period(prime) == prime - 1 {
            long_primes[long_count] = prime;
            long_count += 1;
        }
    }

    for i in 0..long_count {
        if long_primes[i] > numbers[index] {
            totals[index] = count;
            index += 1;
        }
        count += 1;
    }
    totals[numbers.len() - 1] = count;

    println!("The long primes up to {} are:", numbers[0]);
    print!("[");
    for i in 0..totals[0] {
        print!("{} ", long_primes[i]);
    }
    println!("\x08]");

    println!("\nThe number of long primes up to:");
    for (i, &number) in numbers.iter().enumerate() {
        println!("{:5} is {}", number, totals[i]);
    }
}