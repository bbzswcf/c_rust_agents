use std::mem;

#[derive(PartialEq, Copy, Clone)] // Modified: Added Copy and Clone traits to Action enum
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
        let var = self.sum2 / num - mean * mean; // No change needed here, multiplication is correct
        if action == Action::Var {
            return var;
        }
        let stddev = var.sqrt();
        if action == Action::StdDev {
            return stddev;
        }
        0.0
    }

    fn add(&mut self, v: f64) -> f64 {
        self.num += 1;
        self.sum += v;
        self.sum2 += v * v;
        // Modified: Release mutable borrow of `self` before calling `value`
        let action = self.action; // No need to clone or copy since Action implements Copy
        self.value(action)
    }
}

fn main() {
    let v = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let mut so = StatObject::new(Action::StdDev);

    for &val in v.iter() {
        println!("val: {}  std dev: {}", val, so.add(val));
    }
}