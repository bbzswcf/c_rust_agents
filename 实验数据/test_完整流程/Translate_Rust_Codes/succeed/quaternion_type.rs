use std::f64::consts::SQRT_2;

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

    fn copy(&self) -> Box<Self> {
        Box::new(Quaternion { q: self.q })
    }

    fn norm(&self) -> f64 {
        let mut r = 0.0;
        for i in 0..4 {
            r += self.q[i] * self.q[i];
        }
        r.sqrt()
    }

    fn neg(&self) -> Box<Self> {
        Box::new(Quaternion {
            q: [-self.q[0], -self.q[1], -self.q[2], -self.q[3]],
        })
    }

    fn conj(&self) -> Box<Self> {
        Box::new(Quaternion {
            q: [self.q[0], -self.q[1], -self.q[2], -self.q[3]],
        })
    }

    fn add_d(&self, d: f64) -> Box<Self> {
        Box::new(Quaternion {
            q: [self.q[0] + d, self.q[1], self.q[2], self.q[3]],
        })
    }

    fn add(&self, b: &Quaternion) -> Box<Self> {
        Box::new(Quaternion {
            q: [
                self.q[0] + b.q[0],
                self.q[1] + b.q[1],
                self.q[2] + b.q[2],
                self.q[3] + b.q[3],
            ],
        })
    }

    fn mul_d(&self, d: f64) -> Box<Self> {
        Box::new(Quaternion {
            q: [
                self.q[0] * d,
                self.q[1] * d,
                self.q[2] * d,
                self.q[3] * d,
            ],
        })
    }

    fn equal(&self, b: &Quaternion) -> bool {
        let epsilon = 1e-9;
        for i in 0..4 {
            if (self.q[i] - b.q[i]).abs() > epsilon {
                return false;
            }
        }
        true
    }

    fn mul(&self, b: &Quaternion) -> Box<Self> {
        Box::new(Quaternion {
            q: [
                self.q[0] * b.q[0] - self.q[1] * b.q[1] - self.q[2] * b.q[2] - self.q[3] * b.q[3],
                self.q[0] * b.q[1] + self.q[1] * b.q[0] + self.q[2] * b.q[3] - self.q[3] * b.q[2],
                self.q[0] * b.q[2] - self.q[1] * b.q[3] + self.q[2] * b.q[0] + self.q[3] * b.q[1],
                self.q[0] * b.q[3] + self.q[1] * b.q[2] - self.q[2] * b.q[1] + self.q[3] * b.q[0],
            ],
        })
    }

    fn print(&self) {
        println!("({:.6}, {:.6}, {:.6}, {:.6})", self.q[0], self.q[1], self.q[2], self.q[3]);
    }
}

fn main() {
    let d = 7.0;
    let q: Vec<Box<Quaternion>> = vec![
        Quaternion::new_set(1.0, 2.0, 3.0, 4.0),
        Quaternion::new_set(2.0, 3.0, 4.0, 5.0),
        Quaternion::new_set(3.0, 4.0, 5.0, 6.0),
    ];
    let qd = Quaternion::new_set(7.0, 0.0, 0.0, 0.0);

    println!("r = {:.6}", d);

    for i in 0..3 {
        print!("q[{}] = ", i);
        q[i].print();
        println!("abs q[{}] = {:.6}", i, q[i].norm());
    }

    print!("-q[0] = ");
    q[0].neg().print();

    print!("conj q[0] = ");
    q[0].conj().print();

    print!("q[1] + q[2] = ");
    q[1].add(&q[2]).print();

    print!("q[2] + q[1] = ");
    q[2].add(&q[1]).print();

    print!("q[0] * r = ");
    q[0].mul_d(d).print();

    print!("q[0] * (r, 0, 0, 0) = ");
    q[0].mul(&qd).print();

    print!("q[1] * q[2] = ");
    q[1].mul(&q[2]).print();

    print!("q[2] * q[1] = ");
    q[2].mul(&q[1]).print();
}