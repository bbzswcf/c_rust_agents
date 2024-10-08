fn sieve(limit: usize, primes: &mut Vec<i32>, count: &mut usize) {
    let mut c = vec![false; limit + 1];
    let mut p = 2;
    while p * p <= limit {
        if !c[p] {
            for i in (p * p..=limit).step_by(p) {
                c[i] = true;
            }
        }
        p += 1;
    }
    let mut n = 0;
    for i in 2..=limit {
        if !c[i] {
            primes.push(i as i32); // Modified: Push primes into the vector instead of using a fixed index
            n += 1;
        }
    }
    *count = n;
}

fn find_period(n: i32) -> i32 {
    let mut r = 1;
    let mut period = 0;
    let mut seen = vec![0; n as usize];
    while period < n {
        r = (10 * r) % n;
        period += 1;
        if r == 1 {
            return period;
        }
        if seen[r as usize] != 0 {
            return period - seen[r as usize];
        }
        seen[r as usize] = period;
    }
    period
}

fn main() {
    let numbers = vec![500, 1000, 2000, 4000, 8000, 16000, 32000];
    let number_count = numbers.len();
    let mut primes = Vec::new(); // Modified: Use a dynamic vector
    let mut prime_count = 0;
    let mut totals = vec![0; number_count];
    sieve(32000, &mut primes, &mut prime_count);
    let mut long_primes = Vec::with_capacity(prime_count);
    let mut long_count = 0;

    for i in 0..prime_count {
        let prime = primes[i];
        if find_period(prime) == prime - 1 {
            long_primes.push(prime);
            long_count += 1;
        }
    }

    let mut count = 0;
    let mut index = 0;
    for i in 0..long_count {
        if long_primes[i] > numbers[index] {
            totals[index] = count;
            index += 1;
            if index >= number_count {
                break;
            }
        }
        count += 1;
    }
    totals[number_count - 1] = count;

    println!("The long primes up to {} are:", numbers[0]);
    print!("[");
    for i in 0..long_count {
        if long_primes[i] > numbers[0] {
            break;
        }
        if i != 0 {
            print!(" ");
        }
        print!("{}", long_primes[i]);
    }
    println!("]");

    println!("\nThe number of long primes up to:");
    for i in 0..number_count {
        println!("  {:5} is {}", numbers[i], totals[i]);
    }
}