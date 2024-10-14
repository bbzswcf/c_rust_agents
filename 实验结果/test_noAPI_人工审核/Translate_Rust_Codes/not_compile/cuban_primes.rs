use std::alloc::{self, Layout};
use std::ptr;
use std::mem;
use std::io::{self, Write};

type Llong = i64;

struct PrimeArray {
    ptr: *mut Llong,
    size: usize,
    capacity: usize,
}

impl PrimeArray {
    fn allocate() -> Self {
        let capacity = 10;
        let layout = Layout::array::<Llong>(capacity * mem::size_of::<Llong>()).unwrap(); // Modified: Corrected the use of Layout::array
        let ptr = unsafe { alloc::alloc(layout) as *mut Llong };
        if ptr.is_null() {
            panic!("Failed to allocate memory for prime array.");
        }
        PrimeArray {
            ptr,
            size: 0,
            capacity,
        }
    }

    fn deallocate(&mut self) {
        let layout = Layout::array::<Llong>(self.capacity * mem::size_of::<Llong>()).unwrap(); // Modified: Corrected the use of Layout::array
        if !self.ptr.is_null() { // Added: Ensure the pointer is not null before dereferencing
            unsafe { alloc::dealloc(self.ptr as *mut u8, layout); }
        }
        self.ptr = ptr::null_mut();
    }

    fn push_back(&mut self, p: Llong) {
        if self.size >= self.capacity {
            let new_capacity = self.capacity.saturating_mul(3).saturating_div(2).saturating_add(1); // Modified: Ensure no overflow in capacity calculation
            let new_layout = Layout::array::<Llong>(new_capacity * mem::size_of::<Llong>()).unwrap(); // Modified: Corrected the use of Layout::array
            let layout = Layout::array::<Llong>(self.capacity * mem::size_of::<Llong>()).unwrap(); // Modified: Corrected the use of Layout::array
            let new_ptr = unsafe { alloc::realloc(self.ptr as *mut u8, layout, new_capacity * mem::size_of::<Llong>()) as *mut Llong }; // Modified: Use original layout for realloc
            if new_ptr.is_null() {
                panic!("Failed to reallocate the prime array.");
            } else {
                self.ptr = new_ptr;
                self.capacity = new_capacity;
            }
        }
        if self.ptr.is_null() {
            panic!("Pointer is null"); // Added: Ensure the pointer is not null before dereferencing
        }
        unsafe { self.ptr.add(self.size).write(p); } // Added: Ensure the write is within an unsafe block
        self.size += 1;
    }
}

fn main() -> io::Result<()> {
    const CUTOFF: i64 = 200;
    const BIGUN: i64 = 10000;
    const CHUNKS: i64 = 50;
    const LITTLE: i64 = BIGUN / CHUNKS;

    let mut primes = PrimeArray::allocate();
    let mut c = 0;
    let mut show_each = true;
    let mut u = 0;
    let mut v = 1;

    primes.push_back(3);
    primes.push_back(5);

    println!("The first {} cuban primes:", CUTOFF);
    for i in 1..1000000 {
        let mut found = false;
        v += u + 6;
        u += 6;
        let mx: i64 = ((v as f64).sqrt().ceil() as i64); // Modified: Ensure the sqrt and ceil are correctly used

        for j in 0..primes.size {
            if primes.ptr.is_null() {
                panic!("Pointer is null"); // Added: Ensure the pointer is not null before dereferencing
            }
            if j < primes.size && unsafe { *primes.ptr.add(j) } > mx {
                break;
            }
            if j < primes.size && v % unsafe { *primes.ptr.add(j) } == 0 {
                found = true;
                break;
            }
        }
        if !found {
            c += 1;
            if show_each {
                if primes.size > 0 {
                    for z in (unsafe { *primes.ptr.add(primes.size - 1) } + 2)..(v - 2) {
                        let mut fnd = false;
                        for j in 0..primes.size {
                            if primes.ptr.is_null() {
                                panic!("Pointer is null"); // Added: Ensure the pointer is not null before dereferencing
                            }
                            if j < primes.size && unsafe { *primes.ptr.add(j) } > mx {
                                break;
                            }
                            if j < primes.size && z % unsafe { *primes.ptr.add(j) } == 0 {
                                fnd = true;
                                break;
                            }
                        }
                        if !fnd {
                            primes.push_back(z);
                        }
                    }
                }
                primes.push_back(v);
                print!("{:11}", v);
                if c % 10 == 0 {
                    println!();
                }
                if c == CUTOFF {
                    show_each = false;
                    println!("\nProgress to the {}th cuban prime: ", BIGUN);
                }
            }
            if c % LITTLE == 0 {
                print!(".");
                io::stdout().flush()?; // Modified: Ensure the result of flush is handled correctly
                if c == BIGUN {
                    break;
                }
            }
        }
    }
    println!("\nThe {}th cuban prime is {}", c, v); // Modified: Ensure the format string matches the number of arguments

    primes.deallocate();
    Ok(())
}