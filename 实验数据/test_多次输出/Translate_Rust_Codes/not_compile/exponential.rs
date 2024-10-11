use std::mem;
use std::ptr;
use std::fmt;

type SeqFunc = fn(&mut dyn Seq) -> i32;

trait Seq {
    fn next(&mut self) -> i32;
    fn output(&self) -> i32;
    fn set_output(&mut self, value: i32);
}

struct Gen {
    f: SeqFunc,
    output: i32,
}

impl Gen {
    fn new(f: SeqFunc) -> Self {
        Self { f, output: -1 }
    }
}

impl Seq for Gen {
    fn next(&mut self) -> i32 {
        self.output = (self.f)(self);
        self.output
    }

    fn output(&self) -> i32 {
        self.output
    }

    fn set_output(&mut self, value: i32) {
        self.output = value;
    }
}

struct PowerGen {
    base: Gen,
    pos: i32,
    n: i32,
}

impl PowerGen {
    fn new(n: i32) -> Self {
        Self {
            base: Gen::new(Self::power_next),
            pos: -1,
            n,
        }
    }

    fn power_next(&mut self) -> i32 {
        self.pos += 1;
        (self.pos as f64).powi(self.n).round() as i32
    }
}

impl Seq for PowerGen {
    fn next(&mut self) -> i32 {
        self.base.next()
    }

    fn output(&self) -> i32 {
        self.base.output()
    }

    fn set_output(&mut self, value: i32) {
        self.base.set_output(value);
    }
}

struct FilterGen {
    base: Gen,
    in_seq: *mut dyn Seq,
    without_seq: *mut dyn Seq,
}

impl FilterGen {
    fn new(in_seq: &mut dyn Seq, without_seq: &mut dyn Seq) -> Self {
        let in_seq_ptr = in_seq as *mut dyn Seq;
        let without_seq_ptr = without_seq as *mut dyn Seq;
        Self {
            base: Gen::new(Self::filter_next),
            in_seq: in_seq_ptr,
            without_seq: without_seq_ptr,
        }
    }

    fn filter_next(&mut self) -> i32 {
        let in_seq = unsafe { &mut *self.in_seq };
        let without_seq = unsafe { &mut *self.without_seq };

        loop {
            in_seq.next();
            while without_seq.output() < in_seq.output() {
                without_seq.next();
            }
            if without_seq.output() != in_seq.output() {
                break;
            }
        }

        in_seq.output()
    }
}

impl Seq for FilterGen {
    fn next(&mut self) -> i32 {
        self.base.next()
    }

    fn output(&self) -> i32 {
        self.base.output()
    }

    fn set_output(&mut self, value: i32) {
        self.base.set_output(value);
    }
}

fn main() {
    let mut power_seq_2 = PowerGen::new(2);
    let mut power_seq_3 = PowerGen::new(3);
    let mut filter_seq = FilterGen::new(&mut power_seq_2, &mut power_seq_3);

    for _ in 0..20 {
        filter_seq.next();
    }

    for _ in 0..10 {
        println!("{}", filter_seq.next());
    }
}