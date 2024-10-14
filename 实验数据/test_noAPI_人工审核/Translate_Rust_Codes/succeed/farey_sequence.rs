use std::mem;
use std::ptr;

#[derive(Clone)] // Modified: Implement Clone trait for Frac struct
struct Frac {
    d: i32,
    n: i32,
}

fn farey(n: i32) {
    let mut f1 = Frac { d: 0, n: 1 };
    let mut f2 = Frac { d: 1, n: n };
    let mut t;
    let mut k;

    print!("{}/{} {}/{}", f1.d, f1.n, f2.d, f2.n);
    while f2.n > 1 {
        k = (n + f1.n) / f2.n;
        t = f1;
        f1 = f2.clone(); // Modified: Clone f2 before moving it
        f2 = Frac { d: f2.d * k - t.d, n: f2.n * k - t.n };
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
            // Modified: Cast CACHE to *mut u8 before realloc and cast back to *mut ULL
            CACHE = realloc(CACHE as *mut u8, mem::size_of::<ULL>() * CCAP) as *mut ULL;
            if CACHE.is_null() { // Modified: Check for null pointer after realloc
                panic!("Failed to reallocate memory");
            }
            ptr::write_bytes(CACHE.add(old), 0, CCAP - old);
        }

        if !CACHE.is_null() && !CACHE.add(n as usize).is_null() && *CACHE.add(n as usize) != 0 {
            // Modified: Ensure CACHE is not null before dereferencing
            return *CACHE.add(n as usize);
        }

        let mut len = (n as ULL) * (n as ULL + 3) / 2;
        let mut p = 2;
        let mut q;
        while p <= n {
            q = n / (n / p) + 1;
            len -= farey_len(n / p) * (q - p) as ULL;
            p = q;
        }

        if !CACHE.is_null() {
            *CACHE.add(n as usize) = len;
        }
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

extern "C" {
    fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
}