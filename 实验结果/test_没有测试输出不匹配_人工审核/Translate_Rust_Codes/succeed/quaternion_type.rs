use std::f64::consts::PI;

struct Quaternion {
    q: [f64; 4],
}

impl Quaternion {
    fn new() -> Box<Self> {
        Box::new(Quaternion { q: [0.0; 4] })
    }

    fn new_set(q1: f64, q2: f64, q3: f64, q4: f64) -> Box<Self> {
        Box::new(Quaternion { q: [q1, q2, q3, q4] })
    }

    fn copy(&mut self, q: &Quaternion) {
        for i in 0..4 {
            self.q[i] = q.q[i];
        }
    }

    fn norm(&self) -> f64 {
        let mut r = 0.0;
        for i in 0..4 {
            r += self.q[i] * self.q[i];
        }
        r.sqrt()
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

    fn add_d(&mut self, q: &Quaternion, d: f64) {
        self.copy(q);
        self.q[0] += d;
    }

    fn add(&mut self, a: &Quaternion, b: &Quaternion) {
        for i in 0..4 {
            self.q[i] = a.q[i] + b.q[i];
        }
    }

    fn mul_d(&mut self, q: &Quaternion, d: f64) {
        for i in 0..4 {
            self.q[i] = q.q[i] * d;
        }
    }

    fn equal(&self, b: &Quaternion) -> bool {
        for i in 0..4 {
            if self.q[i] != b.q[i] {
                return false;
            }
        }
        true
    }

    fn mul(&mut self, a: &Quaternion, b: &Quaternion) {
        self.q[0] = a.q[0] * b.q[0] - a.q[1] * b.q[1] - a.q[2] * b.q[2] - a.q[3] * b.q[3];
        self.q[1] = a.q[0] * b.q[1] + a.q[1] * b.q[0] + a.q[2] * b.q[3] - a.q[3] * b.q[2];
        self.q[2] = a.q[0] * b.q[2] - a.q[1] * b.q[3] + a.q[2] * b.q[0] + a.q[3] * b.q[1];
        self.q[3] = a.q[0] * b.q[3] + a.q[1] * b.q[2] - a.q[2] * b.q[1] + a.q[3] * b.q[0];
    }

    fn print(&self) {
        println!("({}, {}, {}, {})", self.q[0], self.q[1], self.q[2], self.q[3]);
    }
}

fn main() {
    let d = 7.0;
    let mut q: Vec<Box<Quaternion>> = vec![
        Quaternion::new_set(1.0, 2.0, 3.0, 4.0),
        Quaternion::new_set(2.0, 3.0, 4.0, 5.0),
        Quaternion::new_set(3.0, 4.0, 5.0, 6.0),
    ];
    let mut r = Quaternion::new();
    let qd = Quaternion::new_set(7.0, 0.0, 0.0, 0.0);

    println!("r = {}", d);

    for i in 0..3 {
        print!("q[{}] = ", i);
        q[i].print();
        println!("abs q[{}] = {}", i, q[i].norm());
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