use std::fmt;

type FrIntT = i64;

#[derive(Clone, Copy)]
struct Frac {
    num: FrIntT,
    den: FrIntT,
}

impl Frac {
    fn new(num: FrIntT, den: FrIntT) -> Self {
        if den == 0 {
            eprintln!("divide by zero: {} / {}", num, den);
            std::process::abort();
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

    fn add(self, other: Self) -> Self {
        Frac::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }

    fn sub(self, other: Self) -> Self {
        Frac::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        )
    }

    fn mul(self, other: Self) -> Self {
        Frac::new(self.num * other.num, self.den * other.den)
    }

    fn div(self, other: Self) -> Self {
        Frac::new(self.num * other.den, self.den * other.num)
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let l = self.num * other.den;
        let r = self.den * other.num;
        l.cmp(&r)
    }

    fn cmp_int(&self, other: FrIntT) -> std::cmp::Ordering {
        self.cmp(&Frac::new(other, 1))
    }

    fn to_int(&self) -> FrIntT {
        self.den / self.num
    }

    fn to_double(&self) -> f64 {
        self.den as f64 / self.num as f64
    }
}

fn gcd(mut m: FrIntT, mut n: FrIntT) -> FrIntT {
    while n != 0 {
        let t = n;
        n = m % n;
        m = t;
    }
    m
}

fn main() {
    for n in 2..100000 {
        let mut sum = Frac::new(1, n);

        let mut k = 2;
        while k * k < n {
            if n % k == 0 {
                let kf = Frac::new(1, k);
                sum = sum.add(kf);

                let kf = Frac::new(1, n / k);
                sum = sum.add(kf);
            }
            k += 1;
        }

        if sum.cmp_int(1) == std::cmp::Ordering::Equal {
            println!("{}", n);
        }
    }
}