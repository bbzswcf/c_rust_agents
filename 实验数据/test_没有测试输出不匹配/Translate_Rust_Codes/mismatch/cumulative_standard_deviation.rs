use std::f64::consts::E;

// Modified: Implement the PartialEq trait for the Action enum to allow comparison between enum variants.
#[derive(Debug, PartialEq, Clone)]
enum Action {
    StdDev,
    Mean,
    Var,
    Count,
}

#[derive(Debug)]
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

    fn value(&self, action: Action) -> f64 {
        if self.num == 0 {
            return 0.0;
        }
        let num = self.num as f64;
        if action == Action::Count {
            return num;
        }
        let mean = self.sum / num;
        if action == Action::Mean {
            return mean;
        }
        let var = self.sum2 / num - mean * mean;
        if action == Action::Var {
            return var;
        }
        // Modified: Ensure the sqrt method is available for f64 by directly calling f64::sqrt(var).
        let stddev = f64::sqrt(var);
        if action == Action::StdDev {
            return stddev;
        }
        0.0
    }

    fn add(&mut self, v: f64) -> f64 {
        self.num += 1;
        self.sum += v;
        self.sum2 += v * v;
        // Modified: Ensure the value method is correctly defined and implemented for the StatObject struct.
        self.value(self.action.clone())
    }
}

fn main() {
    let v = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let mut so = StatObject::new(Action::StdDev);

    for &val in v.iter() {
        println!("val: {}  std dev: {}", val, so.add(val));
    }
}