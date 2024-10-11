use std::ptr;
use std::mem;

const BYTES: usize = 256;

struct HuffCode {
    nbits: i32,
    code: i32,
}

struct HuffHeap {
    h: Vec<i32>,
    n: usize,
    s: usize,
    cs: usize,
    f: Vec<i64>,
}

fn heap_create(s: usize, f: &[i64]) -> Box<HuffHeap> {
    let mut h = Box::new(HuffHeap {
        h: Vec::with_capacity(s),
        n: 0,
        s,
        cs: s,
        f: f.to_vec(),
    });
    h.h.resize(s, 0);
    h
}

fn heap_destroy(_heap: Box<HuffHeap>) {}

fn heap_sort(heap: &mut HuffHeap) {
    let mut i = 1;
    let mut j = 2;
    let a = &mut heap.h;

    // Ensure i and i - 1 are within valid bounds
    while i < heap.n && i > 0 {
        if heap.f[a[i - 1] as usize] >= heap.f[a[i] as usize] {
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

fn heap_add(heap: &mut HuffHeap, c: i32) {
    // Ensure heap.n is within the bounds of heap.h
    if heap.n + 1 > heap.s {
        heap.h.resize(heap.s + heap.cs, 0);
        heap.s += heap.cs;
    }
    heap.h[heap.n] = c;
    heap.n += 1;
    heap_sort(heap);
}

fn heap_remove(heap: &mut HuffHeap) -> Option<i32> {
    // Return an Option to handle the case where heap is empty
    if heap.n > 0 {
        heap.n -= 1;
        Some(heap.h[heap.n])
    } else {
        None
    }
}

fn create_huffman_codes(freqs: &[i64; BYTES]) -> Vec<Option<Box<HuffCode>>> {
    let mut codes = Vec::with_capacity(BYTES);
    for _ in 0..BYTES {
        codes.push(None);
    }

    let mut heap = heap_create(BYTES * 2, freqs);
    let mut efreqs = [0i64; BYTES * 2];
    let mut preds = [0i32; BYTES * 2];
    let mut extf = BYTES;

    unsafe {
        ptr::copy_nonoverlapping(freqs.as_ptr(), efreqs.as_mut_ptr(), BYTES);
        ptr::write_bytes(efreqs.as_mut_ptr().add(BYTES), 0, BYTES);
    }

    for i in 0..BYTES {
        if efreqs[i] > 0 {
            heap_add(&mut heap, i as i32);
        }
    }

    // Ensure heap.n is greater than 1 before attempting to remove elements
    while heap.n > 1 {
        if let Some(r1) = heap_remove(&mut heap) {
            if let Some(r2) = heap_remove(&mut heap) {
                efreqs[extf] = efreqs[r1 as usize] + efreqs[r2 as usize];
                heap_add(&mut heap, extf as i32);
                // Ensure r1 and r2 are within the bounds of preds
                if r1 >= 0 && r1 < preds.len() as i32 && r2 >= 0 && r2 < preds.len() as i32 {
                    preds[r1 as usize] = extf as i32;
                    preds[r2 as usize] = -(extf as i32);
                }
                extf += 1;
            }
        }
    }

    if let Some(r1) = heap_remove(&mut heap) {
        // Ensure r1 is within the bounds of preds
        if r1 >= 0 && r1 < preds.len() as i32 {
            preds[r1 as usize] = r1;
        }
    }
    heap_destroy(heap);

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
        codes[i] = Some(Box::new(HuffCode {
            nbits: bn,
            code: bc,
        }));
    }

    codes
}

fn free_huffman_codes(codes: Vec<Option<Box<HuffCode>>>) {
    // Automatically managed
}

const MAXBITSPERCODE: usize = 100;

fn inttobits(c: i32, n: i32, s: &mut [char; MAXBITSPERCODE]) {
    let mut c = c;
    let mut n = n;
    // Ensure n is within the bounds of s
    if (n as usize) < MAXBITSPERCODE { // Modified: Enclose the cast in parentheses
        s[n as usize] = '\0';
    }
    // Ensure n - 1 is within the bounds of s
    while n > 0 && n <= MAXBITSPERCODE as i32 {
        if ((n - 1) as usize) < MAXBITSPERCODE { // Modified: Enclose the cast in parentheses
            s[(n - 1) as usize] = (c.checked_rem(2).unwrap_or(0) + '0' as i32) as u8 as char;
        }
        c >>= 1;
        n -= 1;
    }
}

const TEST: &str = "this is an example for huffman encoding";

fn main() {
    let mut freqs = [0i64; BYTES];
    let mut p = TEST.chars();

    while let Some(ch) = p.next() {
        // Ensure ch is within the bounds of freqs
        if (ch as usize) < freqs.len() {
            freqs[ch as usize] += 1;
        }
    }

    let codes = create_huffman_codes(&freqs);

    for i in 0..BYTES {
        if let Some(code) = &codes[i] {
            let mut strbit = ['\0'; MAXBITSPERCODE];
            inttobits(code.code, code.nbits, &mut strbit);
            println!("{} ({}) {}", i as u8 as char, code.code, strbit.iter().collect::<String>());
        }
    }

    free_huffman_codes(codes);
}