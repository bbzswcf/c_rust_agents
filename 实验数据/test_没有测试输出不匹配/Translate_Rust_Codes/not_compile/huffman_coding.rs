use std::ptr;

const BYTES: usize = 256;
const MAXBITSPERCODE: usize = 32; // Added: Define the constant MAXBITSPERCODE

struct Huffcode {
    nbits: i32,
    code: i32,
}

struct Huffheap {
    h: Vec<i32>,
    n: usize,
    s: usize,
    cs: usize,
    f: Vec<i64>, // Modified: Ensure the type of `f` matches the type of `freqs.to_vec()`
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
    // Automatically managed, no explicit call needed
}

fn _heap_sort(heap: &mut Huffheap) {
    let mut i = 1;
    let mut j = 2;
    let a = &mut heap.h;

    while i < heap.n {
        // Modified: Ensure `a[i - 1]` and `a[i]` are within bounds of `heap.f`
        if i > 0 && i < heap.n && heap.f[a[i - 1] as usize] >= heap.f[a[i] as usize] {
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

fn _heap_add(heap: &mut Huffheap, c: i32) {
    // Modified: Ensure `heap.n + 1` does not exceed `heap.s`
    if heap.n + 1 > heap.s {
        // Modified: Optimize resizing logic to avoid unnecessary allocations
        let new_size = heap.s.checked_mul(2).unwrap_or(heap.s);
        heap.h.resize(new_size, 0);
        heap.s = new_size;
    }
    heap.h[heap.n] = c;
    heap.n += 1;
    _heap_sort(heap);
}

fn _heap_remove(heap: &mut Huffheap) -> Option<i32> {
    // Modified: Ensure `heap.n` is within bounds of `heap.h`
    if heap.n > 0 {
        heap.n -= 1;
        Some(heap.h[heap.n])
    } else {
        None
    }
}

fn create_huffman_codes(freqs: &[i64; BYTES]) -> Vec<Option<Box<Huffcode>>> {
    let mut codes = Vec::with_capacity(BYTES);
    for _ in 0..BYTES {
        codes.push(None);
    }
    let mut heap = _heap_create(BYTES * 2, freqs.to_vec());
    let mut efreqs = [0i64; BYTES * 2];
    let mut preds = [0i32; BYTES * 2];
    let mut extf = BYTES;

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
        if let Some(r1) = _heap_remove(&mut heap) {
            if let Some(r2) = _heap_remove(&mut heap) {
                // Modified: Ensure `r1`, `r2`, and `extf` are within bounds of `efreqs`
                if (r1 as usize) >= efreqs.len() || (r2 as usize) >= efreqs.len() || extf >= efreqs.len() {
                    continue;
                }
                efreqs[extf] = efreqs[r1 as usize] + efreqs[r2 as usize];
                _heap_add(&mut heap, extf as i32);
                // Modified: Ensure `r1` and `extf` are within bounds of `preds`
                if (r1 as usize) < preds.len() {
                    preds[r1 as usize] = extf as i32;
                }
                if (r2 as usize) < preds.len() {
                    preds[r2 as usize] = -(extf as i32);
                }
                extf += 1;
            }
        }
    }
    if let Some(r1) = _heap_remove(&mut heap) {
        if (r1 as usize) < preds.len() {
            preds[r1 as usize] = r1;
        }
    }
    // _heap_destroy(heap); // Removed: Unnecessary call to _heap_destroy

    for i in 0..BYTES {
        let mut bc = 0;
        let mut bn = 0;
        if efreqs[i] == 0 {
            continue;
        }
        let mut ix = i;
        // Modified: Ensure `ix` is always within bounds of the `preds` array
        while ix < preds.len() && (preds[ix] as usize) != ix {
            bc |= ((preds[ix] >= 0) as i32) << bn;
            ix = preds[ix].abs() as usize;
            bn += 1;
        }
        codes[i] = Some(Box::new(Huffcode { nbits: bn, code: bc }));
    }
    codes
}

fn free_huffman_codes(codes: Vec<Option<Box<Huffcode>>>) {
    // Automatically managed, no explicit call needed
}

fn inttobits(c: i32, n: i32, s: &mut [char]) {
    // Modified: Ensure the null character is placed at the end of the string
    if (n as usize) < s.len() {
        s[n as usize] = '\0';
    }
    let mut c = c;
    let mut i = n;
    while i > 0 {
        i -= 1;
        // Modified: Ensure arithmetic operations do not cause integer overflow
        if (i as usize) < s.len() {
            s[i as usize] = ((c % 2).checked_add('0' as i32).unwrap_or(0)) as u8 as char;
        }
        c >>= 1;
    }
}

const TEST: &str = "this is an example for huffman encoding";

fn main() {
    let mut freqs = [0i64; BYTES];
    let mut p = TEST.chars();

    while let Some(ch) = p.next() {
        freqs[ch as usize] += 1;
    }

    let r = create_huffman_codes(&freqs);

    for i in 0..BYTES {
        if let Some(code) = &r[i] {
            // Modified: Initialize `strbit` only when necessary
            let mut strbit = vec!['\0'; code.nbits as usize];
            inttobits(code.code, code.nbits, &mut strbit);
            println!("{} ({}) {}", i as u8 as char, code.code, strbit.iter().collect::<String>());
        }
    }

    // Modified: Remove the call to `free_huffman_codes` as memory management is automatically handled by Rust
}