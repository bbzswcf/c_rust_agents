use std::process;

type FrIntT = i64;

#[derive(Clone, Copy)]
struct Frac {
    num: FrIntT,
    den: FrIntT,
}

fn gcd(mut m: FrIntT, mut n: FrIntT) -> FrIntT {
    while n != 0 {
        let t = n;
        n = m % n;
        m = t;
    }
    m
}

fn frac_new(num: FrIntT, den: FrIntT) -> Frac {
    if den == 0 {
        println!("divide by zero: {} / {}", num, den);
        process::abort();
    }

    let g = gcd(num, den);

    let (num, den) = if g != 0 {
        (num / g, den / g)
    } else {
        (0, 1)
    };

    let (num, den) = if den < 0 {
        (-num, -den)
    } else {
        (num, den)
    };

    Frac { num, den }
}

fn frac_add(a: Frac, b: Frac) -> Frac {
    frac_new(a.num * b.den + b.num * a.den, a.den * b.den)
}

fn frac_sub(a: Frac, b: Frac) -> Frac {
    frac_new(a.num * b.den - b.num * a.den, a.den * b.den)
}

fn frac_mul(a: Frac, b: Frac) -> Frac {
    frac_new(a.num * b.num, a.den * b.den)
}

fn frac_div(a: Frac, b: Frac) -> Frac {
    frac_new(a.num * b.den, a.den * b.num)
}

fn frac_cmp(a: Frac, b: Frac) -> i32 {
    let l = a.num * b.den;
    let r = a.den * b.num;
    if l < r {
        -1
    } else if l > r {
        1
    } else {
        0
    }
}

fn frac_cmp_int(a: Frac, b: FrIntT) -> i32 {
    frac_cmp(a, frac_new(b, 1))
}

fn frtoi(a: Frac) -> i32 {
    (a.den / a.num) as i32
}

fn frtod(a: Frac) -> f64 {
    a.den as f64 / a.num as f64
}

fn main() {
    for n in 2..100000 {
        let mut sum = frac_new(1, n);

        let mut k = 2;
        while k * k < n {
            if n % k == 0 {
                let kf = frac_new(1, k);
                sum = frac_add(sum, kf);

                let kf = frac_new(1, n / k);
                sum = frac_add(sum, kf);
            }
            k += 1;
        }
        if frac_cmp_int(sum, 1) == 0 {
            println!("{}", n);
        }
    }
}