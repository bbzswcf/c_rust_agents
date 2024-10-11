use std::boxed::Box;

trait Seq {
    fn next(&mut self) -> i32;
}

struct Gen {
    f: Box<dyn FnMut(&mut Gen) -> i32>,
    output: i32,
}

impl Seq for Gen {
    fn next(&mut self) -> i32 {
        // Modified: Split the mutable borrow into two separate statements
        let f = &mut self.f;
        self.output = f(self);
        self.output
    }
}

struct PowerGen {
    base: Gen,
    pos: i32,
    n: i32,
}

impl Seq for PowerGen {
    fn next(&mut self) -> i32 {
        self.pos += 1;
        self.base.output = (self.pos as f64).powi(self.n) as i32;
        self.base.output
    }
}

fn power_seq(n: i32) -> Box<dyn Seq> {
    let mut gen = PowerGen {
        base: Gen {
            f: Box::new(|_| 0),
            output: -1,
        },
        pos: -1,
        n,
    };
    // Modified: Use dynamic dispatch by storing a closure that calls the appropriate method
    gen.base.f = Box::new(|s| {
        let s = s as &mut PowerGen;
        s.next()
    });
    Box::new(gen)
}

struct FilterGen {
    base: Gen,
    in_seq: Box<dyn Seq>,
    without_seq: Box<dyn Seq>,
}

impl Seq for FilterGen {
    fn next(&mut self) -> i32 {
        loop {
            let in_next = self.in_seq.next();
            let mut without_next = self.without_seq.next();
            while without_next < in_next {
                without_next = self.without_seq.next();
            }
            if without_next != in_next {
                break;
            }
        }
        self.in_seq.next()
    }
}

fn filter_seq(in_seq: Box<dyn Seq>, without_seq: Box<dyn Seq>) -> Box<dyn Seq> {
    let mut gen = FilterGen {
        base: Gen {
            f: Box::new(|_| 0),
            output: -1,
        },
        in_seq,
        without_seq,
    };
    // Modified: Use dynamic dispatch by storing a closure that calls the appropriate method
    gen.base.f = Box::new(|s| {
        let s = s as &mut FilterGen;
        s.next()
    });
    Box::new(gen)
}

fn main() {
    let mut s = filter_seq(power_seq(2), power_seq(3));

    for _ in 0..20 {
        s.next();
    }
    for _ in 0..10 {
        println!("{}", s.next());
    }
}