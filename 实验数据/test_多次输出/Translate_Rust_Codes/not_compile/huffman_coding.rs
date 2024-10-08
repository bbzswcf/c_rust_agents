use std::mem;
use std::ptr;

const BYTES: usize = 256;

#[derive(Debug)]
struct Huffcode {
    nbits: i32,
    code: i32,
}

#[derive(Debug)]
struct Huffheap {
    h: Vec<i32>,
    n: usize,
    s: usize,
    cs: usize,
    f: Vec<i64>,
}

impl Huffheap {
    fn new(s: usize, f: Vec<i64>) -> Self {
        Self {
            h: vec![0; s],
            n: 0,
            s,
            cs: s,
            f,
        }
    }

    fn destroy(&mut self) {
        self.h.clear();
        self.f.clear();
    }

    fn sort(&mut self) {
        let mut i = 1;
        let mut j = 2;
        let a = &mut self.h;

        while i < self.n {
            if self.f[a[i - 1] as usize] >= self.f[a[i] as usize] {
                i = j;
                j += 1;
            } else {
                a.swap(i - 1, i);
                i -= 1;
                i = if i == 0 { j } else { i };
                j += 1;
            }
        }
    }

    fn add(&mut self, c: i32) {
        if self.n + 1 > self.s {
            self.h.resize(self.s + self.cs, 0);
            self.s += self.cs;
        }
        self.h[self.n] = c;
        self.n += 1;
        self.sort();
    }

    fn remove(&mut self) -> i32 {
        if self.n > 0 {
            self.n -= 1;
            self.h[self.n]
        } else {
            -1
        }
    }
}

fn create_huffman_codes(freqs: &[i64; BYTES]) -> Vec<Option<Box<Huffcode>>> {
    let mut codes = vec![None; BYTES];
    let mut heap = Huffheap::new(BYTES * 2, freqs.to_vec());
    let mut efreqs = [0; BYTES * 2];
    let mut preds = [0; BYTES * 2];
    let mut extf = BYTES;

    efreqs[..BYTES].copy_from_slice(freqs);

    for i in 0..BYTES {
        if efreqs[i] > 0 {
            heap.add(i as i32);
        }
    }

    while heap.n > 1 {
        let r1 = heap.remove();
        let r2 = heap.remove();
        efreqs[extf] = efreqs[r1 as usize] + efreqs[r2 as usize];
        heap.add(extf as i32);
        preds[r1 as usize] = extf as i32;
        preds[r2 as usize] = -(extf as i32);
        extf += 1;
    }

    let r1 = heap.remove();
    preds[r1 as usize] = r1;
    heap.destroy();

    for i in 0..BYTES {
        if efreqs[i] == 0 {
            continue;
        }

        let mut bc = 0;
        let mut bn = 0;
        let mut ix = i;

        while preds[ix].abs() != ix as i32 {
            bc |= ((preds[ix] >= 0) as i32) << bn;
            ix = preds[ix].abs() as usize;
            bn += 1;
        }

        codes[i] = Some(Box::new(Huffcode {
            nbits: bn,
            code: bc,
        }));
    }

    codes
}

fn free_huffman_codes(codes: Vec<Option<Box<Huffcode>>>) {
    for code in codes {
        if let Some(c) = code {
            drop(c);
        }
    }
}

fn int_to_bits(c: i32, n: usize, s: &mut [u8]) {
    s[n] = 0;
    for i in 0..n {
        s[n - 1 - i] = ((c >> i) & 1) as u8 + b'0';
    }
}

const TEST: &str = "this is an example for huffman encoding";

fn main() {
    let mut freqs = [0; BYTES];

    for &ch in TEST.as_bytes() {
        freqs[ch as usize] += 1;
    }

    let codes = create_huffman_codes(&freqs);

    for i in 0..BYTES {
        if let Some(code) = &codes[i] {
            let mut strbit = vec![0; code.nbits as usize + 1];
            int_to_bits(code.code, code.nbits as usize, &mut strbit);
            println!(
                "{} ({}) {}",
                i as u8 as char,
                code.code,
                std::str::from_utf8(&strbit).unwrap()
            );
        }
    }

    free_huffman_codes(codes);
}