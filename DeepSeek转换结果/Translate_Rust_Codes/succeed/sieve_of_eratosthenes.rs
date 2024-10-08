fn eratosthenes(n: i32) -> Option<(Vec<u8>, i32)> {
    if n < 2 {
        return None;
    }

    let mut primes_count = n - 1;
    let m = (n as f64).sqrt() as i32;

    let mut sieve = vec![0; (n + 1) as usize];
    sieve[0] = 1;
    sieve[1] = 1;

    for i in 2..=m {
        if sieve[i as usize] == 0 {
            for j in (i * i..=n).step_by(i as usize) {
                if sieve[j as usize] == 0 {
                    sieve[j as usize] = 1;
                    primes_count -= 1;
                }
            }
        }
    }

    Some((sieve, primes_count))
}

fn sieve(a: &mut Vec<i32>, n: i32) {
    for i in 2..=n {
        a[i as usize] = 1;
    }

    for i in 2..=n {
        print!("\ni:{}", i);
        if a[i as usize] == 1 {
            for j in i.. {
                let index = i * j;
                if index > n {
                    break;
                }
                print!("\nj:{}", j);
                print!("\nBefore a[{}*{}]: {}", i, j, a[index as usize]);
                a[index as usize] = 0;
                print!("\nAfter a[{}*{}]: {}", i, j, a[index as usize]);
            }
        }
    }

    print!("\nPrimes numbers from 1 to {} are : ", n);
    for i in 2..=n {
        if a[i as usize] == 1 {
            print!("{}, ", i);
        }
    }
    print!("\n\n");
}

fn main() {
    let n = 10;
    let mut array = vec![0; (n + 1) as usize];
    sieve(&mut array, n);
}