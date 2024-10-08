use std::any::Any;
use std::cell::RefCell;

type SeqFunc = fn(&mut dyn Seq) -> i32;

trait Seq: Any {
    fn output(&self) -> i32;
    fn set_output(&mut self, value: i32);
    fn func(&self) -> SeqFunc;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct Gen {
    f: SeqFunc,
    output: i32,
}

impl Seq for Gen {
    fn output(&self) -> i32 {
        self.output
    }

    fn set_output(&mut self, value: i32) {
        self.output = value;
    }

    fn func(&self) -> SeqFunc {
        self.f
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn seq_next(state: &mut dyn Seq) -> i32 {
    let func = state.func();
    // 
    let new_output = func(state);
    state.set_output(new_output);
    state.output()
}

struct PowerGen {
    base: Gen,
    pos: i32,
    n: i32,
}

impl Seq for PowerGen {
    fn output(&self) -> i32 {
        self.base.output()
    }

    fn set_output(&mut self, value: i32) {
        self.base.set_output(value);
    }

    fn func(&self) -> SeqFunc {
        self.base.func()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// :  &mut PowerGen  &mut dyn Seq
fn power_next(s: &mut PowerGen) -> i32 {
    s.pos += 1;
    (s.pos as f64).powi(s.n) as i32
}

fn power_seq(n: i32) -> Box<dyn Seq> {
    let mut s = Box::new(PowerGen {
        base: Gen {
            f: |s: &mut dyn Seq| power_next(s.as_any_mut().downcast_mut::<PowerGen>().unwrap()), // :  downcast_mut 
            output: -1,
        },
        pos: -1,
        n,
    });
    s.set_output(-1);
    s
}

struct FilterGen {
    base: Gen,
    in_seq: RefCell<Box<dyn Seq>>,
    without_seq: RefCell<Box<dyn Seq>>,
}

impl Seq for FilterGen {
    fn output(&self) -> i32 {
        self.base.output()
    }

    fn set_output(&mut self, value: i32) {
        self.base.set_output(value);
    }

    fn func(&self) -> SeqFunc {
        self.base.func()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// :  &mut FilterGen  &mut dyn Seq
fn filter_next(s: &mut FilterGen) -> i32 {
    // 修改: 使用 `*` 解引用 `RefMut` 并获取 `&mut Box<dyn Seq>`，然后再次解引用 `Box` 以获取 `&mut dyn Seq`
    let in_seq = &mut **s.in_seq.borrow_mut();
    let without_seq = &mut **s.without_seq.borrow_mut();

    loop {
        // :  seq_next
        seq_next(in_seq);
        while without_seq.output() < in_seq.output() {
            seq_next(without_seq);
        }
        if without_seq.output() != in_seq.output() {
            break;
        }
    }

    in_seq.output()
}

fn filter_seq(in_seq: Box<dyn Seq>, without_seq: Box<dyn Seq>) -> Box<dyn Seq> {
    let mut filt = Box::new(FilterGen {
        base: Gen {
            f: |s: &mut dyn Seq| filter_next(s.as_any_mut().downcast_mut::<FilterGen>().unwrap()), // :  downcast_mut 
            output: -1,
        },
        in_seq: RefCell::new(in_seq),
        without_seq: RefCell::new(without_seq),
    });
    filt.set_output(-1);
    filt
}

fn main() {
    let mut s = filter_seq(power_seq(2), power_seq(3));

    for _ in 0..20 {
        // :  seq_next
        seq_next(&mut *s);
    }
    for _ in 0..10 {
        // :  seq_next
        println!("{:?}", seq_next(&mut *s));
    }
}