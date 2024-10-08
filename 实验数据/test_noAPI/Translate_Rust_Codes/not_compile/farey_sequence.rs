use std::ptr;
use std::alloc::{alloc, realloc, Layout};

struct Frac {
    d: i32,
    n: i32,
}

// Modified: Implement the `Clone` trait for `Frac`
impl Clone for Frac {
    fn clone(&self) -> Self {
        Frac { d: self.d, n: self.n }
    }
}

fn farey(n: i32) {
    let mut f1 = Frac { d: 0, n: 1 };
    // Modified: Remove unnecessary type casting
    let mut f2 = Frac { d: 1, n };
    let mut k;

    print!("{}/{} {}/{}", f1.d, f1.n, f2.d, f2.n);
    // Modified: Correct the loop condition to avoid infinite loop
    while f2.n > 1 {
        k = (n + f1.n) / f2.n;
        // Modified: Directly assign `f1 = f2` to avoid unnecessary clone
        f1 = f2;
        f2 = Frac { d: f2.d * k - f1.d, n: f2.n * k - f1.n };
        print!(" {}/{}", f2.d, f2.n);
    }

    println!();
}

type ULL = u64;

static mut CACHE: *mut ULL = ptr::null_mut();
static mut CCAP: usize = 0;

fn farey_len(n: i32) -> ULL {
    unsafe {
        if n as usize >= CCAP {
            let old = CCAP;
            if CCAP == 0 {
                CCAP = 16;
            }
            while CCAP <= n as usize {
                CCAP *= 2;
            }
            // Modified: Use Rust's standard library functions for memory allocation
            let layout = Layout::array::<ULL>(CCAP).unwrap();
            if CACHE.is_null() {
                CACHE = alloc(layout) as *mut ULL;
            } else {
                CACHE = realloc(CACHE as *mut u8, layout, layout.size()) as *mut ULL;
            }
            if CACHE.is_null() {
                panic!("Memory allocation failed");
            }
            ptr::write_bytes(CACHE.add(old), 0, CCAP - old);
        }

        // Modified: Ensure `CACHE` is properly initialized before dereferencing it
        if CACHE.is_null() {
            return 0;
        }

        if *CACHE.add(n as usize) != 0 {
            return *CACHE.add(n as usize);
        }

        let mut len = (n as ULL) * (n as ULL + 3) / 2;
        let mut p = 2;
        let mut q;
        while p <= n {
            q = n / (n / p) + 1;
            // Modified: Ensure operations do not cause integer overflow
            len = len.saturating_sub(farey_len(n / p) * (q - p) as ULL);
            p = q;
        }

        *CACHE.add(n as usize) = len;
        len
    }
}

fn main() {
    for n in 1..=11 {
        print!("{}: ", n);
        farey(n);
    }

    for n in (100..=1000).step_by(100) {
        println!("{}: {} items", n, farey_len(n));
    }

    let n = 10000000;
    println!("\n{}: {} items", n, farey_len(n));
}