fn farey(n: i32) {
    #[derive(Clone, Copy)] //  Frac  Copy trait
    struct Frac { d: i32, n: i32 }
    let mut f1 = Frac { d: 0, n: 1 };
    //  f2  1/n,n  i32 
    let mut f2 = Frac { d: 1, n: n };
    let mut t;
    let mut k;

    print!("0/1");
    while f2.n > 1 {
        k = (n + f1.n) / f2.n;
        t = f1; //  Frac  Copy trait,
        f1 = f2;
        f2 = Frac { d: f2.d * k - t.d, n: f2.n * k - t.n };
        print!(" {}/{}", f2.d, f2.n);
    }
    print!(" 1/1\n"); // : C
}

struct Cache {
    data: Vec<u64>,
    capacity: usize,
}

impl Cache {
    fn new() -> Self {
        Cache { data: Vec::new(), capacity: 0 }
    }

    fn resize(&mut self, new_capacity: usize) {
        if new_capacity > self.capacity {
            self.data.resize(new_capacity, 0); //  0
            self.capacity = new_capacity;
        }
    }

    fn get(&self, n: usize) -> Option<u64> {
        if n < self.capacity {
            Some(self.data[n])
        } else {
            None
        }
    }

    fn set(&mut self, n: usize, value: u64) -> Result<(), &'static str> {
        if n < self.capacity {
            self.data[n] = value;
            Ok(())
        } else {
            Err("Index out of bounds") //  panic
        }
    }
}

fn farey_len(n: i32, cache: &mut Cache) -> u64 {
    //  n  usize 
    let n_usize = n as usize;
    if n_usize >= cache.capacity {
        let old = cache.capacity;
        let mut new_capacity = if cache.capacity == 0 { 16 } else { cache.capacity };
        while new_capacity <= n_usize {
            new_capacity *= 2;
        }
        cache.resize(new_capacity);
        //  0
        for i in old..new_capacity {
            cache.data[i] = 0;
        }
    } else if let Some(cached_value) = cache.get(n_usize) {
        return cached_value;
    }

    let mut len = (n as u64) * (n as u64 + 3) / 2;
    let mut p = 2;
    let mut q;
    while p <= n {
        q = n / (n / p) + 1;
        len -= farey_len(n / p, cache) * (q - p) as u64;
        p = q;
    }

    //  unwrap_or_else 
    cache.set(n_usize, len).unwrap_or_else(|_| panic!("Failed to set cache value"));
    len
}

fn main() {
    let mut cache = Cache::new();

    for n in 1..=11 {
        print!("{}: ", n);
        farey(n);
    }

    for n in (100..=1000).step_by(100) {
        println!("{}: {} items", n, farey_len(n, &mut cache));
    }

    let n = 10000000;
    println!("\n{}: {} items", n, farey_len(n, &mut cache));
}