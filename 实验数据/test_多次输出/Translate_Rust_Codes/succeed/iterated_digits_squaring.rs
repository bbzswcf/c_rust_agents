fn is_89(mut x: usize) -> bool {
    loop {
        let mut s = 0;
        while x > 0 {
            let digit = x % 10;
            s += digit * digit;
            x /= 10;
        }

        if s == 89 {
            return true;
        }
        if s == 1 {
            return false;
        }
        x = s;
    }
}

fn main() {
    const MAX_SUM: usize = 32 * 81 + 1;
    let mut sums = vec![0; MAX_SUM];
    sums[0] = 1;

    for n in 1.. {
        for i in (1..=n * 81).rev() {
            for j in 1..10 {
                let s = j * j;
                if s > i {
                    break;
                }
                sums[i] += sums[i - s];
            }
        }

        let mut count_89 = 0;
        for i in 1..=n * 81 {
            if !is_89(i) {
                continue;
            }

            if sums[i] > !0_u64 - count_89 {
                println!("counter overflow for 10^{}", n);
                return;
            }
            count_89 += sums[i];
        }

        println!("1->10^{}: {}", n, count_89);
    }
}