fn is89(mut x: i32) -> bool {
    loop {
        let mut s = 0;
        let mut x_copy = x;
        while x_copy != 0 {
            let digit = x_copy % 10;
            s += digit * digit;
            x_copy /= 10;
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
    let mut sums: [u64; 32 * 81 + 1] = [0; 32 * 81 + 1];
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

        let mut count89: u64 = 0;
        for i in 1..=n * 81 {
            if !is89(i as i32) {
                continue;
            }

            if sums[i] > !0u64 - count89 {
                print!("counter overflow for 10^{}\n", n);
                return;
            }
            count89 += sums[i];
        }

        print!("1->10^{}: {}\n", n, count89);
    }
}