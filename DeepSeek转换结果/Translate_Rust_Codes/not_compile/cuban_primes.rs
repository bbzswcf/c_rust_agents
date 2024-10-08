use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr;
use std::sync::Mutex;
use std::io::Write; // 修改: 引入Write trait以支持writeln!宏

type Llong = i64;

struct PrimeArray {
    ptr: *mut Llong,
    size: usize,
    capacity: usize,
}

impl PrimeArray {
    fn allocate() -> Self {
        let capacity = 10;
        // 修改: 使用unwrap_or_else处理Result
        let layout = Layout::array::<Llong>(capacity).unwrap_or_else(|_| {
            eprintln!("Failed to create layout");
            std::process::exit(1);
        });
        let ptr = unsafe { alloc(layout) as *mut Llong };
        PrimeArray {
            ptr,
            size: 0,
            capacity,
        }
    }

    fn deallocate(&mut self) {
        // 修改: 使用unwrap_or_else处理Result
        let layout = Layout::array::<Llong>(self.capacity).unwrap_or_else(|_| {
            eprintln!("Failed to create layout");
            std::process::exit(1);
        });
        unsafe { dealloc(self.ptr as *mut u8, layout) };
        self.ptr = ptr::null_mut();
    }

    fn push_back(&mut self, p: Llong) {
        if self.size >= self.capacity {
            let new_capacity = (3 * self.capacity) / 2 + 1;
            // 修改: 使用unwrap_or_else处理Result
            let new_layout = Layout::array::<Llong>(new_capacity).unwrap_or_else(|_| {
                eprintln!("Failed to create layout");
                std::process::exit(1);
            });
            // 修改: 使用unwrap_or_else处理Result
            let old_layout = Layout::array::<Llong>(self.capacity).unwrap_or_else(|_| {
                eprintln!("Failed to create layout");
                std::process::exit(1);
            });
            let old_ptr = self.ptr;
            let temp = unsafe { realloc(self.ptr as *mut u8, old_layout, new_layout.size()) as *mut Llong };
            if temp.is_null() {
                unsafe { dealloc(old_ptr as *mut u8, old_layout) };
                eprint!("Failed to reallocate the prime array.");
                std::process::exit(1);
            } else {
                self.ptr = temp;
                self.capacity = new_capacity;
            }
        }
        // 修改: 使用unwrap_or_else处理Result
        if self.size >= self.capacity {
            eprint!("Failed to push back element, array is full.");
            std::process::exit(1);
        }
        unsafe { *self.ptr.add(self.size) = p };
        self.size += 1;
    }
}

fn main() {
    const CUTOFF: i32 = 200;
    const BIGUN: i32 = 10000;
    const CHUNKS: i32 = 50;
    const LITTLE: i32 = BIGUN / CHUNKS;

    // 修改: 使用Mutex保护共享资源
    let primes = Mutex::new(PrimeArray::allocate());
    let mut c = 0;
    let mut show_each = true;
    let mut u = 0;
    let mut v = 1;

    {
        let mut primes = primes.lock().unwrap_or_else(|_| {
            eprintln!("Failed to lock primes");
            std::process::exit(1);
        });
        primes.push_back(3);
        primes.push_back(5);
    }

    // 修改: 使用unwrap_or_else处理Result
    writeln!(std::io::stdout(), "The first {} cuban primes:\n", CUTOFF).unwrap_or_else(|_| {
        eprintln!("Failed to write to stdout");
        std::process::exit(1);
    });
    let mut i = 1;
    while i < Llong::MAX {
        let mut found = false;
        v += u + 6;
        u += 6;
        let mx = (v as f64).sqrt().ceil().max(Llong::MIN as f64).min(Llong::MAX as f64) as Llong;

        let primes = primes.lock().unwrap_or_else(|_| {
            eprintln!("Failed to lock primes");
            std::process::exit(1);
        });
        for j in 0..primes.size {
            if unsafe { *primes.ptr.add(j) } > mx {
                break;
            }
            if v % unsafe { *primes.ptr.add(j) } == 0 {
                found = true;
                break;
            }
        }
        if !found {
            c += 1;
            if show_each {
                for z in (unsafe { *primes.ptr.add(primes.size - 1) } + 2)..(v - 2) {
                    let mut fnd = false;
                    for j in 0..primes.size {
                        if unsafe { *primes.ptr.add(j) } > mx {
                            break;
                        }
                        if z % unsafe { *primes.ptr.add(j) } == 0 {
                            fnd = true;
                            break;
                        }
                    }
                    if !fnd {
                        primes.push_back(z);
                    }
                }
                primes.push_back(v);
                print!("{:11}", v);
                if c % 10 == 0 {
                    print!("\n");
                }
                if c == CUTOFF {
                    show_each = false;
                    print!("\nProgress to the {}th cuban prime: ", BIGUN);
                }
            }
            if c % LITTLE == 0 {
                print!(".");
                if c == BIGUN {
                    break;
                }
            }
        }
        i += 1;
    }
    // 修改: 使用unwrap_or_else处理Result
    writeln!(std::io::stdout(), "\nThe {}th cuban prime is {}\n", c, v).unwrap_or_else(|_| {
        eprintln!("Failed to write to stdout");
        std::process::exit(1);
    });

    // 修改: 实现Drop trait以自动释放资源
    impl Drop for PrimeArray {
        fn drop(&mut self) {
            self.deallocate();
        }
    }
}