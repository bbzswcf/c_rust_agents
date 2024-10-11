fn is_perfect(n: i32) -> bool {
    let max = (n as f64).sqrt() as i32 + 1;
    let mut tot = 1;

    for i in 2..max {
        if n % i == 0 {
            tot += i;
            let q = n / i;
            if q > i {
                tot += q;
            }
        }
    }

    tot == n
}

fn main() {
    for n in 2..200 {
        if is_perfect(n) {
            println!("{}", n);
        }
    }
}