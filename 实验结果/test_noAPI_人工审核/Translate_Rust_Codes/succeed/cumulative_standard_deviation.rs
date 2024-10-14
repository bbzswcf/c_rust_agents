use std::mem;

#[derive(Clone, Copy, PartialEq)] // Added: Derive PartialEq to allow comparison operations
enum Action {
    StdDev,
    Mean,
    Var,
    Count,
}

struct StatObject {
    sum: f64,
    sum2: f64,
    num: usize,
    action: Action,
}

impl StatObject {
    fn new(action: Action) -> Self {
        StatObject {
            sum: 0.0,
            sum2: 0.0,
            num: 0,
            action,
        }
    }

    fn value(&self) -> f64 {
        if self.num == 0 {
            return f64::NAN; // Ensures NaN is returned when number of elements is zero to avoid division by zero
        }
        let num = self.num as f64;
        if self.action == Action::Count {
            return self.num as f64;
        }
        let mean = self.sum / num;
        if self.action == Action::Mean {
            return mean;
        }
        let var = (self.sum2 / num) - (self.sum / num).powi(2); // Corrected variance calculation to avoid precision issues
        if self.action == Action::Var {
            return var;
        }
        let epsilon = 1e-10;
        let stddev = if var.abs() < epsilon { 0.0 } else { var.sqrt() }; // Ensures floating-point comparison is appropriate
        if self.action == Action::StdDev {
            return stddev;
        }
        f64::NAN
    }

    fn add(&mut self, v: f64) -> f64 {
        self.num += 1;
        self.sum += v;
        self.sum2 += v * v;
        self.value()
    }
}

fn main() {
    let v = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let mut so = StatObject::new(Action::StdDev);

    for &val in v.iter() {
        // Modified: Format `val` as a floating-point number with six decimal places
        println!("val: {:.6}  std dev: {:.6}", val, so.add(val));
    }
}