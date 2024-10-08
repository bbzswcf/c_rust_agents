#![allow(dead_code)]

const BYTES: usize = 256;
const MAXBITSPERCODE: usize = 100;

struct Huffcode {
    nbits: i32,
    code: i32,
}

struct Huffheap {
    h: Vec<i32>,
    n: usize,
    s: usize,
    cs: usize,
    f: Vec<i64>,
}

fn _heap_create(s: usize, f: Vec<i64>) -> Option<Box<Huffheap>> {
    // 修改: 使用 Vec::with_capacity 和 resize_with 初始化 h
    let mut h = Vec::with_capacity(s);
    h.resize_with(s, Default::default);
    Some(Box::new(Huffheap {
        h,
        n: 0,
        s,
        cs: s,
        f,
    }))
}

fn _heap_destroy(_heap: Option<Box<Huffheap>>) {
    // Drop automatically handles deallocation
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
        // 修改: 使用 Vec::resize_with 扩展 h
        heap.h.resize_with(heap.s + heap.cs, Default::default);
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
        // 修改: 当 heap.n 为 0 时返回 -1 而不是 panic
        -1
    }
}

fn create_huffman_codes(freqs: &[i64; BYTES]) -> Vec<Option<Box<Huffcode>>> {
    // 修改: 使用 Vec::with_capacity 和 resize_with 初始化 codes
    let mut codes = Vec::with_capacity(BYTES);
    codes.resize_with(BYTES, || None);

    let mut heap = match _heap_create(BYTES * 2, freqs.to_vec()) {
        Some(heap) => heap,
        None => return codes, // 修改: 当 _heap_create 返回 None 时直接返回 codes
    };
    let mut efreqs = [0i64; BYTES * 2];
    let mut preds = [0i32; BYTES * 2];
    let mut extf = BYTES;

    unsafe {
        std::ptr::copy_nonoverlapping(freqs.as_ptr(), efreqs.as_mut_ptr(), BYTES);
        std::ptr::write_bytes(efreqs.as_mut_ptr().add(BYTES), 0, BYTES);
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
    _heap_destroy(Some(heap));

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
        codes[i] = Some(Box::new(Huffcode { nbits: bn, code: bc }));
    }
    codes
}

fn free_huffman_codes(codes: Vec<Option<Box<Huffcode>>>) {
    // Drop automatically handles deallocation
}

fn inttobits(c: i32, n: usize, s: &mut [char; MAXBITSPERCODE]) {
    // 修改: 确保 n 不超过 MAXBITSPERCODE
    if n < MAXBITSPERCODE {
        s[n] = '\0';
    }
    let mut n = n;
    let mut c = c;
    while n > 0 {
        n -= 1;
        if n < MAXBITSPERCODE {
            s[n] = ((c % 2) + '0' as i32) as u8 as char;
        }
        c >>= 1;
    }
}

fn main() {
    let test = "this is an example for huffman encoding";
    let mut freqs = [0i64; BYTES];

    for &c in test.as_bytes() {
        freqs[c as usize] += 1;
    }

    let r = create_huffman_codes(&freqs);

    let mut strbit = ['\0'; MAXBITSPERCODE];
    for i in 0..BYTES {
        if let Some(code) = &r[i] {
            inttobits(code.code, code.nbits as usize, &mut strbit);
            print!("{} ({}) {}\n", i as u8 as char, code.code, strbit.iter().collect::<String>());
        }
    }

    free_huffman_codes(r);
}