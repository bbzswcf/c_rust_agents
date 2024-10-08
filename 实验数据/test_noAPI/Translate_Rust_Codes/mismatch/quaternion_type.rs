use std::f64::consts::SQRT_2;

#[derive(Debug, Clone, PartialEq)]
struct Quaternion {
    q: [f64; 4],
}

impl Quaternion {
    fn new() -> Self {
        Quaternion { q: [0.0; 4] }
    }

    fn new_set(q1: f64, q2: f64, q3: f64, q4: f64) -> Self {
        Quaternion { q: [q1, q2, q3, q4] }
    }

    fn copy(&mut self, q: &Quaternion) {
        for i in 0..4 {
            self.q[i] = q.q[i]; // Ensure each component is copied from the source quaternion
        }
    }

    fn norm(&self) -> f64 {
        let mut r = 0.0;
        for i in 0..4 {
            r += self.q[i] * self.q[i]; // Sum of the squares of each component
        }
        r.sqrt() // Take the square root of the sum
    }

    fn neg(&mut self, q: &Quaternion) {
        for i in 0..4 {
            self.q[i] = -q.q[i]; // Negate all components of the quaternion
        }
    }

    fn conj(&mut self, q: &Quaternion) {
        self.q[0] = q.q[0]; // Real part remains unchanged
        for i in 1..4 {
            self.q[i] = -q.q[i]; // Negate all imaginary parts
        }
    }

    fn add_d(&mut self, q: &Quaternion, d: f64) {
        self.q[0] = q.q[0] + d; // Add the scalar `d` to the real part of the quaternion
        for i in 1..4 {
            self.q[i] = q.q[i]; // Copy the imaginary parts as they are
        }
    }

    fn add(&mut self, a: &Quaternion, b: &Quaternion) {
        for i in 0..4 {
            self.q[i] = a.q[i] + b.q[i]; // Element-wise addition of each component
        }
    }

    fn mul_d(&mut self, q: &Quaternion, d: f64) {
        for i in 0..4 {
            self.q[i] = q.q[i] * d; // Multiply all components by the scalar `d`
        }
    }

    fn equal(&self, b: &Quaternion) -> bool {
        const EPSILON: f64 = 1e-9;
        for i in 0..4 {
            if (self.q[i] - b.q[i]).abs() > EPSILON {
                return false; // Compare all components with a tolerance for floating-point precision errors
            }
        }
        true
    }

    fn mul(&mut self, a: &Quaternion, b: &Quaternion) {
        self.q[0] = a.q[0] * b.q[0] - a.q[1] * b.q[1] - a.q[2] * b.q[2] - a.q[3] * b.q[3];
        self.q[1] = a.q[0] * b.q[1] + a.q[1] * b.q[0] + a.q[2] * b.q[3] - a.q[3] * b.q[2];
        self.q[2] = a.q[0] * b.q[2] - a.q[1] * b.q[3] + a.q[2] * b.q[0] + a.q[3] * b.q[1];
        self.q[3] = a.q[0] * b.q[3] + a.q[1] * b.q[2] - a.q[2] * b.q[1] + a.q[3] * b.q[0];
        // Correct quaternion multiplication logic with correct signs and indices
    }

    fn print(&self) {
        println!("({}, {}, {}, {})", self.q[0], self.q[1], self.q[2], self.q[3]);
    }
}

fn main() {
    let d = 7.0;
    let mut q = vec![
        Quaternion::new_set(1.0, 2.0, 3.0, 4.0),
        Quaternion::new_set(2.0, 3.0, 4.0, 5.0),
        Quaternion::new_set(3.0, 4.0, 5.0, 6.0),
    ];
    let mut r = Quaternion::new();
    let qd = Quaternion::new_set(7.0, 0.0, 0.0, 0.0);

    println!("r = {}", d);

    for (i, qi) in q.iter().enumerate() {
        print!("q[{}] = ", i);
        qi.print();
        println!("abs q[{}] = {}", i, qi.norm());
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
    r.mul_d(&q[0], d);
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