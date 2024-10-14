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

fn _heap_create(s: usize, f: Vec<i64>) -> Box<Huffheap> {
    let mut h = Box::new(Huffheap {
        h: Vec::with_capacity(s),
        n: 0,
        s,
        cs: s,
        f,
    });
    h.h.resize(s, 0);
    h
}

fn _heap_destroy(_heap: Box<Huffheap>) {
    // Automatically managed by Rust
}

fn _heap_sort(heap: &mut Huffheap) {
    let mut i = heap.n / 2;
    while i > 0 {
        i -= 1;
        let mut k = i;
        let v = heap.h[k];
        let mut heapify = false;
        while !heapify {
            let mut j = 2 * k + 1;
            if j < heap.n {
                if j + 1 < heap.n && heap.f[heap.h[j + 1] as usize] < heap.f[heap.h[j] as usize] {
                    j += 1;
                }
                if heap.f[v as usize] > heap.f[heap.h[j] as usize] {
                    heap.h[k] = heap.h[j];
                    k = j;
                } else {
                    heapify = true;
                }
            } else {
                heapify = true;
            }
        }
        heap.h[k] = v;
    }
}

fn _heap_add(heap: &mut Huffheap, c: i32) {
    if heap.n + 1 > heap.s {
        heap.h.resize(heap.s + heap.cs, 0);
        heap.s += heap.cs;
    }
    heap.h[heap.n] = c;
    heap.n += 1;
    _heap_sort(heap);
}

fn _heap_remove(heap: &mut Huffheap) -> i32 {
    if heap.n > 0 {
        let root = heap.h[0];
        heap.n -= 1;
        heap.h[0] = heap.h[heap.n];
        _heap_sort(heap);
        root
    } else {
        -1
    }
}

fn create_huffman_codes(freqs: &[i64; BYTES]) -> Vec<Option<Box<Huffcode>>> {
    let mut efreqs = [0i64; BYTES * 2];
    let mut preds = [0i32; BYTES * 2];
    let mut extf = BYTES;
    let mut heap = _heap_create(BYTES * 2, efreqs.to_vec());

    unsafe {
        ptr::copy_nonoverlapping(freqs.as_ptr(), efreqs.as_mut_ptr(), BYTES);
        ptr::write_bytes(efreqs.as_mut_ptr().add(BYTES), 0, BYTES);
    }

    for i in 0..BYTES {
        if efreqs[i] > 0 {
            _heap_add(&mut heap, i as i32);
        }
    }

    while heap.n > 1 {
        let r1 = _heap_remove(&mut heap);
        let r2 = _heap_remove(&mut heap);
        efreqs[extf] = efreqs[r1 as usize] + efreqs[r2 as usize];
        _heap_add(&mut heap, extf as i32);
        preds[r1 as usize] = extf as i32;
        preds[r2 as usize] = -(extf as i32);
        extf += 1;
    }

    let r1 = _heap_remove(&mut heap);
    preds[r1 as usize] = r1;
    _heap_destroy(heap);

    let mut codes = Vec::with_capacity(BYTES);
    for _ in 0..BYTES {
        codes.push(None);
    }

    for i in 0..BYTES {
        let mut bc = 0;
        let mut bn = 0;
        if efreqs[i] == 0 {
            continue;
        }
        let mut ix = i;
        while (preds[ix] as usize) != ix {
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
    // Automatically managed by Rust
}

const MAXBITSPERCODE: usize = 100;

fn inttobits(c: i32, n: usize, s: &mut [char; MAXBITSPERCODE]) {
    s[n] = '\0';
    let mut n = n;
    let mut c = c;
    while n > 0 {
        n -= 1;
        s[n] = ((c % 2) as u8 + b'0') as char;
        c >>= 1;
    }
}

const TEST: &str = "this is an example for huffman encoding";

fn main() {
    let mut freqs = [0i64; BYTES];
    let mut strbit = ['\0'; MAXBITSPERCODE];

    for c in TEST.chars() {
        freqs[c as usize] += 1;
    }

    let r = create_huffman_codes(&freqs);

    for i in 0..BYTES {
        if let Some(code) = &r[i] {
            inttobits(code.code, code.nbits as usize, &mut strbit);
            println!("{} ({}) {}", i as u8 as char, code.code, strbit.iter().collect::<String>());
        }
    }

    free_huffman_codes(r);
}