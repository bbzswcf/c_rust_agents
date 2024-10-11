use std::alloc::{alloc, Layout};
use std::ptr;
use std::mem;

type SeqFunc = fn(*mut ()) -> i32;

#[repr(C)]
struct Gen {
    f: SeqFunc,
    output: i32,
}

impl Gen {
    fn next(&mut self) -> i32 {
        // Ensure the cast is correct and matches the expected type
        self.output = (self.f)(self as *mut _ as *mut ());
        self.output
    }
}

#[repr(C)]
struct PowerGen {
    base: Gen,
    pos: i32,
    n: i32,
}

fn power_next(s: *mut ()) -> i32 {
    // Ensure the cast is correct and matches the expected type
    let s = s as *mut PowerGen;
    unsafe {
        (*s).pos += 1;
        (*s).base.output = (*s).pos.pow((*s).n as u32);
        (*s).base.output
    }
}

fn power_seq(n: i32) -> *mut PowerGen {
    unsafe {
        let layout = Layout::new::<PowerGen>();
        let s = alloc(layout) as *mut PowerGen;
        ptr::write(s, PowerGen {
            base: Gen { f: power_next, output: -1 },
            pos: -1,
            n,
        });
        s
    }
}

#[repr(C)]
struct FilterGen {
    base: Gen,
    in_gen: *mut Gen,
    without_gen: *mut Gen,
}

fn filter_next(s: *mut ()) -> i32 {
    // Ensure the cast is correct and matches the expected type
    let s = s as *mut FilterGen;
    unsafe {
        let in_gen = &mut *(*s).in_gen;
        let without_gen = &mut *(*s).without_gen;
        loop {
            in_gen.next();
            while without_gen.output < in_gen.output {
                without_gen.next();
            }
            if without_gen.output != in_gen.output {
                break;
            }
        }
        in_gen.output
    }
}

fn filter_seq(in_gen: *mut Gen, without_gen: *mut Gen) -> *mut FilterGen {
    unsafe {
        let layout = Layout::new::<FilterGen>();
        let filt = alloc(layout) as *mut FilterGen;
        ptr::write(filt, FilterGen {
            base: Gen { f: filter_next, output: -1 },
            in_gen,
            without_gen,
        });
        filt
    }
}

fn main() {
    unsafe {
        let s = filter_seq(power_seq(2), power_seq(3));
        // Ensure the cast is correct and matches the expected type
        let s = &mut *(s as *mut Gen);

        // Ensure the loop range is correct and matches the expected type
        for _ in 0..20 {
            s.next();
        }
        // Ensure the loop range is correct and matches the expected type
        for _ in 0..10 {
            println!("{}", s.next());
        }
    }
}