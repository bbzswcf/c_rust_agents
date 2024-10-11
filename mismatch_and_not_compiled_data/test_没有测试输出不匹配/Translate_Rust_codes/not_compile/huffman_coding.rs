use std::ptr;
use std::mem;

const BYTES: usize = 256;

struct Huffcode {
    nbits: i32,
    code: i32,
}

struct Huffheap {
    h: Vec<i32>,
    n: usize,
    s: usize,
    cs: usize,
    f: [i64; BYTES], // Modified: Changed Vec<i64> to [i64; BYTES] to avoid dynamic allocation
}

fn _heap_create(s: usize, f: [i64; BYTES]) -> Huffheap { // Modified: Removed Box to avoid unnecessary dynamic allocation
    Huffheap {
        h: vec![0; s],
        n: 0,
        s,
        cs: s,
        f,
    }
}

fn _heap_destroy(_heap: Huffheap) {
    // Dropping the Huffheap will automatically free the memory
}

fn _heap_sort(heap: &mut Huffheap) {
    let mut i = 1;
    let mut j = 2;
    let a = &mut heap.h;

    while i < heap.n {
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
        heap.n -= 1;
        heap.h[heap.n]
    } else {
        -1
    }
}

fn create_huffman_codes(freqs: &[i64; BYTES]) -> Vec<Option<Huffcode>> { // Modified: Changed Vec<Option<Box<Huffcode>>> to Vec<Option<Huffcode>>
    let mut codes = Vec::with_capacity(BYTES); // Modified: Used a loop to initialize the vector to avoid cloning
    for _ in 0..BYTES {
        codes.push(None);
    }

    let mut heap = _heap_create(BYTES * 2, *freqs); // Modified: Passed the array directly
    let mut efreqs = [0i64; BYTES * 2];
    let mut preds = [0i32; BYTES * 2];
    let mut extf = BYTES;

    efreqs[..BYTES].copy_from_slice(freqs); // Modified: Used safe slice operations
    efreqs[BYTES..].fill(0); // Modified: Used safe slice operations

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
        codes[i] = Some(Huffcode { nbits: bn, code: bc }); // Modified: Stored Huffcode directly without Box
    }
    codes
}

fn free_huffman_codes(codes: Vec<Option<Huffcode>>) {
    // Dropping the Vec will automatically free the memory
}

const MAXBITSPERCODE: usize = 100;

fn inttobits(c: i32, n: usize, s: &mut [char; MAXBITSPERCODE]) {
    s[n] = '\0';
    let mut n = n;
    let mut c = c;
    while n > 0 {
        n -= 1;
        s[n] = ((c % 2) + '0' as i32) as u8 as char;
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
            let mut strbit = ['\0'; MAXBITSPERCODE];
            inttobits(code.code, code.nbits as usize, &mut strbit);
            println!("{} ({}) {}", i as u8 as char, code.code, strbit.iter().collect::<String>());
        }
    }

    free_huffman_codes(r);
}