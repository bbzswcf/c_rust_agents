use std::boxed::Box;

type SeqFunc = fn(&mut dyn Seq) -> i32;

trait Seq {
    fn next(&mut self) -> i32;
}

struct Gen {
    f: SeqFunc,
    output: i32,
}

impl Seq for Gen {
    fn next(&mut self) -> i32 {
        self.output = (self.f)(self)
    }
}

struct PowerGen {
    f: SeqFunc,
    output: i32,
    pos: i32,
    n: i32,
}

impl Seq for PowerGen {
    fn next(&mut self) -> i32 {
        self.pos += 1;
        self.output = (self.pos as f64).powi(self.n) as i32;
        self.output
    }
}

fn power_seq(n: i32) -> Box<dyn Seq> {
    Box::new(PowerGen {
        f: |_| 0,
        output: -1,
        pos: -1,
        n,
    })
}

struct FilterGen {
    f: SeqFunc,
    output: i32,
    in_seq: Box<dyn Seq>,
    without_seq: Box<dyn Seq>,
}

impl Seq for FilterGen {
    fn next(&mut self) -> i32 {
        loop {
            self.in_seq.next();
            while self.without_seq.next() < self.in_seq.next() {
                self.without_seq.next();
            }
            if self.without_seq.next() != self.in_seq.next() {
                break self.in_seq.next();
            }
        }
    }
}

fn filter_seq(in_seq: Box<dyn Seq>, without_seq: Box<dyn Seq>) -> Box<dyn Seq> {
    Box::new(FilterGen {
        f: |_| 0,
        output: -1,
        in_seq,
        without_seq,
    })
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