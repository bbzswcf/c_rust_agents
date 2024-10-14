fn perfect(n: i32) -> bool {
    let max = (f64::sqrt(n as f64) + 1.0) as i32;
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
        if perfect(n) {
            println!("{}", n);
        }
    }
}