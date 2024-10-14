use std::process;

type FrIntT = i64;

#[derive(Debug)]
struct Frac {
    num: FrIntT,
    den: FrIntT,
}

// Modified: Use checked_rem to handle potential overflow in gcd function
fn gcd(mut m: FrIntT, mut n: FrIntT) -> FrIntT {
    while n != 0 {
        let t = n;
        n = m.checked_rem(n).unwrap_or(0);
        m = t;
    }
    m
}

// Modified: Return Result type to handle division by zero more gracefully
fn frac_new(num: FrIntT, den: FrIntT) -> Result<Frac, &'static str> {
    if den == 0 {
        return Err("divide by zero");
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

    Ok(Frac { num, den })
}

// Modified: Macro now accepts variables a and b and uses them in the expressions
macro_rules! binop {
    ($op:ident, $a:ident, $b:ident, $n:expr, $d:expr) => {
        fn $op($a: Frac, $b: Frac) -> Frac {
            frac_new($n, $d).unwrap_or_else(|e| {
                println!("Error: {}", e);
                process::abort();
            })
        }
    };
}

binop!(frac_add, a, b, a.num * b.den + b.num * a.den, a.den * b.den);
binop!(frac_sub, a, b, a.num * b.den - b.num * a.den, a.den * b.den);
binop!(frac_mul, a, b, a.num * b.num, a.den * b.den);
binop!(frac_div, a, b, a.num * b.den, a.den * b.num);

// Modified: Use checked_mul to handle potential overflow in frac_cmp function
fn frac_cmp(a: Frac, b: Frac) -> i32 {
    let l = a.num.checked_mul(b.den).unwrap_or(FrIntT::MAX);
    let r = a.den.checked_mul(b.num).unwrap_or(FrIntT::MAX);
    if l < r {
        -1
    } else if l > r {
        1
    } else {
        0
    }
}

fn frac_cmp_int(a: Frac, b: FrIntT) -> i32 {
    frac_cmp(a, frac_new(b, 1).unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::abort();
    }))
}

fn frtoi(a: Frac) -> i32 {
    (a.den / a.num) as i32
}

fn frtod(a: Frac) -> f64 {
    a.den as f64 / a.num as f64
}

fn main() {
    for n in 2..100000 {
        let mut sum = frac_new(1, n).unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::abort();
        });

        let mut k = 2;
        while k * k < n {
            if n % k == 0 {
                let kf = frac_new(1, k).unwrap_or_else(|e| {
                    println!("Error: {}", e);
                    process::abort();
                });
                sum = frac_add(sum, kf);

                let kf = frac_new(1, n / k).unwrap_or_else(|e| {
                    println!("Error: {}", e);
                    process::abort();
                });
                sum = frac_add(sum, kf);
            }
            k += 1;
        }
        if frac_cmp_int(sum, 1) == 0 {
            println!("{}", n);
        }
    }
}