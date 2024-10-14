use std::f64::consts::SQRT_2;

#[derive(Debug, Clone, PartialEq)]
struct Quaternion {
    q: [f64; 4],
}

impl Quaternion {
    fn new() -> Self {
        Quaternion { q: [0.0; 4] }
    }

    fn from_components(q1: f64, q2: f64, q3: f64, q4: f64) -> Self {
        Quaternion { q: [q1, q2, q3, q4] }
    }

    fn copy(&mut self, q: &Quaternion) {
        for i in 0..4 {
            self.q[i] = q.q[i];
        }
    }

    fn norm(&self) -> f64 {
        self.q.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }

    fn neg(&mut self, q: &Quaternion) {
        for i in 0..4 {
            self.q[i] = -q.q[i];
        }
    }

    fn conj(&mut self, q: &Quaternion) {
        self.q[0] = q.q[0];
        for i in 1..4 {
            self.q[i] = -q.q[i];
        }
    }

    fn add_scalar(&mut self, q: &Quaternion, d: f64) {
        self.copy(q);
        self.q[0] += d;
    }

    fn add(&mut self, a: &Quaternion, b: &Quaternion) {
        for i in 0..4 {
            self.q[i] = a.q[i] + b.q[i];
        }
    }

    fn mul_scalar(&mut self, q: &Quaternion, d: f64) {
        for i in 0..4 {
            self.q[i] = q.q[i] * d;
        }
    }

    fn equal(&self, other: &Quaternion) -> bool {
        self.q.iter().zip(other.q.iter()).all(|(&a, &b)| a == b)
    }

    fn mul(&mut self, a: &Quaternion, b: &Quaternion) {
        self.q[0] = a.q[0] * b.q[0] - a.q[1] * b.q[1] - a.q[2] * b.q[2] - a.q[3] * b.q[3];
        self.q[1] = a.q[0] * b.q[1] + a.q[1] * b.q[0] + a.q[2] * b.q[3] - a.q[3] * b.q[2];
        self.q[2] = a.q[0] * b.q[2] - a.q[1] * b.q[3] + a.q[2] * b.q[0] + a.q[3] * b.q[1];
        self.q[3] = a.q[0] * b.q[3] + a.q[1] * b.q[2] - a.q[2] * b.q[1] + a.q[3] * b.q[0];
    }

    fn print(&self) {
        println!(
            "({:.6}, {:.6}, {:.6}, {:.6})",
            self.q[0], self.q[1], self.q[2], self.q[3]
        ); // Modified: Format quaternion components with 6 decimal places
    }
}

fn main() {
    let d = 7.0;
    let mut q = vec![
        Quaternion::from_components(1.0, 2.0, 3.0, 4.0),
        Quaternion::from_components(2.0, 3.0, 4.0, 5.0),
        Quaternion::from_components(3.0, 4.0, 5.0, 6.0),
    ];
    let mut r = Quaternion::new();
    let qd = Quaternion::from_components(7.0, 0.0, 0.0, 0.0);

    println!("r = {:.6}", d); // Modified: Format scalar `r` with 6 decimal places

    for (i, qi) in q.iter().enumerate() {
        print!("q[{}] = ", i);
        qi.print();
        println!("abs q[{}] = {:.6}", i, qi.norm()); // Modified: Format norm with 6 decimal places
    }

    print!("-q[0] = ");
    r.neg(&q[0]);
    r.print();

    print!("conj q[0] = ");
    r.conj(&q[0]);
    r.print();

    print!("q[1] + q[2] = ");
    r.add(&q[1], &q[2]);
    r.print();

    print!("q[2] + q[1] = ");
    r.add(&q[2], &q[1]);
    r.print();

    print!("q[0] * r = ");
    r.mul_scalar(&q[0], d);
    r.print();

    print!("q[0] * (r, 0, 0, 0) = ");
    r.mul(&q[0], &qd);
    r.print();

    print!("q[1] * q[2] = ");
    r.mul(&q[1], &q[2]);
    r.print();

    print!("q[2] * q[1] = ");
    r.mul(&q[2], &q[1]);
    r.print();
}