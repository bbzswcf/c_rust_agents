use std::boxed::Box;
use std::fmt::Write;

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
    base: Gen,
    pos: i32,
    n: i32,
}

impl Seq for PowerGen {
    fn next(&mut self) -> i32 {
        self.pos += 1;
        (self.base.f)(self)
    }
}

impl PowerGen {
    fn new(n: i32) -> Box<dyn Seq> {
        Box::new(PowerGen {
            base: Gen {
                f: Self::power_next,
                output: -1,
            },
            pos: -1,
            n,
        })
    }

    fn power_next(s: &mut dyn Seq) -> i32 {
        let s = s.as_any_mut().downcast_mut::<PowerGen>().unwrap();
        s.pos.pow(s.n as u32)
    }
}

struct FilterGen {
    base: Gen,
    in_seq: Box<dyn Seq>,
    without: Box<dyn Seq>,
}

impl Seq for FilterGen {
    fn next(&mut self) -> i32 {
        self.base.next()
    }
}

impl FilterGen {
    fn new(in_seq: Box<dyn Seq>, without: Box<dyn Seq>) -> Box<dyn Seq> {
        Box::new(FilterGen {
            base: Gen {
                f: Self::filter_next,
                output: -1,
            },
            in_seq,
            without,
        })
    }

    fn filter_next(s: &mut dyn Seq) -> i32 {
        let s = s.as_any_mut().downcast_mut::<FilterGen>().unwrap();
        loop {
            s.in_seq.next();
            while s.without.output() < s.in_seq.output() {
                s.without.next();
            }
            if s.without.output() != s.in_seq.output() {
                break s.in_seq.output();
            }
        }
    }
}

trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn main() {
    let mut s = FilterGen::new(PowerGen::new(2), PowerGen::new(3));

    for _ in 0..20 {
        s.next();
    }

    let mut output = String::new();
    for _ in 0..10 {
        writeln!(output, "{}", s.next()).unwrap();
    }

    print!("{}", output);
}