// Added: Ensure the `libc` crate is added as a dependency in `Cargo.toml`.
// [dependencies]
// libc = "0.2"

extern crate libc; // Added: Declare the libc crate for Rust 2015 edition
use std::mem;
use std::ptr;

const BYTES: usize = 256;

struct Huffcode {
    nbits: i32,
    code: i32,
}

struct Huffheap {
    h: *mut i32,
    n: i32,
    s: i32,
    cs: i32,
    f: *mut i64,
}

// Added: Documentation comments to explain the purpose and safety guarantees of each unsafe function.
/// Creates a new Huffheap with the given size and frequency array.
/// # Safety
/// This function uses unsafe operations to allocate and manipulate raw pointers.
/// It is the caller's responsibility to ensure that the input parameters are valid.
unsafe fn heap_create(s: i32, f: *mut i64) -> *mut Huffheap {
    let h = libc::malloc(mem::size_of::<Huffheap>()) as *mut Huffheap;
    if h.is_null() {
        return ptr::null_mut();
    }
    (*h).h = libc::malloc((s as usize) * mem::size_of::<i32>()) as *mut i32;
    if (*h).h.is_null() {
        libc::free(h as *mut libc::c_void); // Free previously allocated memory to avoid memory leak
        return ptr::null_mut(); // Added: Return null pointer if allocation fails
    }
    (*h).s = s;
    (*h).cs = s;
    (*h).n = 0;
    (*h).f = f;
    h
}

/// Destroys the given Huffheap, freeing all associated memory.
/// # Safety
/// This function uses unsafe operations to free raw pointers.
/// It is the caller's responsibility to ensure that the input pointer is valid.
unsafe fn heap_destroy(heap: *mut Huffheap) {
    libc::free((*heap).h as *mut libc::c_void);
    libc::free(heap as *mut libc::c_void);
}

/// Sorts the elements in the given Huffheap.
/// # Safety
/// This function uses unsafe operations to manipulate raw pointers.
/// It is the caller's responsibility to ensure that the input pointer is valid.
unsafe fn heap_sort(heap: *mut Huffheap) {
    let mut i = 1;
    let a = (*heap).h;

    while i < (*heap).n {
        if *(*heap).f.offset(a.offset((i - 1) as isize) as isize) >= *(*heap).f.offset(a.offset(i as isize) as isize) {
            i += 1;
        } else {
            let t = *a.offset((i - 1) as isize);
            *a.offset((i - 1) as isize) = *a.offset(i as isize);
            *a.offset(i as isize) = t;
            i -= 1;
            i = if i == 0 { 1 } else { i };
        }
    }
}

/// Adds an element to the given Huffheap.
/// # Safety
/// This function uses unsafe operations to manipulate raw pointers.
/// It is the caller's responsibility to ensure that the input pointer is valid.
unsafe fn heap_add(heap: *mut Huffheap, c: i32) {
    if (*heap).n + 1 > (*heap).s {
        (*heap).h = libc::realloc((*heap).h as *mut libc::c_void, ((*heap).s + (*heap).cs) as usize * mem::size_of::<i32>()) as *mut i32;
        (*heap).s += (*heap).cs;
    }
    *(*heap).h.offset((*heap).n as isize) = c;
    (*heap).n += 1;
    heap_sort(heap);
}

/// Removes an element from the given Huffheap.
/// # Safety
/// This function uses unsafe operations to manipulate raw pointers.
/// It is the caller's responsibility to ensure that the input pointer is valid.
unsafe fn heap_remove(heap: *mut Huffheap) -> i32 {
    if (*heap).n > 0 { // Ensure (*heap).n is greater than 0 before accessing
        (*heap).n -= 1;
        return *(*heap).h.offset((*heap).n as isize);
    }
    -1
}

/// Creates Huffman codes based on the given frequency array.
/// # Safety
/// This function uses unsafe operations to allocate and manipulate raw pointers.
/// It is the caller's responsibility to ensure that the input parameters are valid.
unsafe fn create_huffman_codes(freqs: *mut i64) -> *mut *mut Huffcode {
    let mut codes: *mut *mut Huffcode = libc::malloc(BYTES * mem::size_of::<*mut Huffcode>()) as *mut *mut Huffcode;
    let mut heap: *mut Huffheap = heap_create((BYTES * 2) as i32, freqs);
    if heap.is_null() {
        return ptr::null_mut();
    }

    let mut efreqs: [i64; BYTES * 2] = [0; BYTES * 2];
    let mut preds: [i32; BYTES * 2] = [0; BYTES * 2];
    let mut extf = BYTES as i32;
    let mut r1: i32;
    let mut r2: i32;

    ptr::copy_nonoverlapping(freqs, efreqs.as_mut_ptr(), BYTES);
    ptr::write_bytes(efreqs.as_mut_ptr().offset(BYTES as isize), 0, BYTES);

    for i in 0..BYTES {
        if *efreqs.as_ptr().offset(i as isize) > 0 {
            heap_add(heap, i as i32);
        }
    }

    while (*heap).n > 1 {
        r1 = heap_remove(heap);
        r2 = heap_remove(heap);
        *efreqs.as_mut_ptr().offset(extf as isize) = *efreqs.as_ptr().offset(r1 as isize) + *efreqs.as_ptr().offset(r2 as isize);
        heap_add(heap, extf);
        preds[r1 as usize] = extf;
        preds[r2 as usize] = -extf;
        extf += 1;
    }
    r1 = heap_remove(heap);
    preds[r1 as usize] = r1;
    heap_destroy(heap);

    for i in 0..BYTES {
        let mut bc = 0;
        let mut bn = 0;
        if *efreqs.as_ptr().offset(i as isize) == 0 {
            *codes.offset(i as isize) = ptr::null_mut();
            continue;
        }
        let mut ix = i as i32;
        while preds[ix as usize].abs() != ix {
            bc |= ((preds[ix as usize] >= 0) as i32) << bn;
            ix = preds[ix as usize].abs();
            bn += 1;
        }
        *codes.offset(i as isize) = libc::malloc(mem::size_of::<Huffcode>()) as *mut Huffcode;
        (*(*codes.offset(i as isize))).nbits = bn;
        (*(*codes.offset(i as isize))).code = bc;
    }
    codes
}

/// Frees the memory allocated for the given Huffman codes.
/// # Safety
/// This function uses unsafe operations to free raw pointers.
/// It is the caller's responsibility to ensure that the input pointer is valid.
unsafe fn free_huffman_codes(c: *mut *mut Huffcode) {
    for i in 0..BYTES {
        if !(*c.offset(i as isize)).is_null() { // Ensure the pointer is not null before calling libc::free
            libc::free(*c.offset(i as isize) as *mut libc::c_void);
        }
    }
    libc::free(c as *mut libc::c_void);
}

const MAXBITSPERCODE: usize = 100;

/// Converts an integer to its binary representation.
/// # Safety
/// This function assumes that the `n` parameter does not exceed the bounds of the `s` array.
fn inttobits(c: i32, n: i32, s: &mut [u8; MAXBITSPERCODE]) {
    if n as usize >= MAXBITSPERCODE { // Added: Ensure the `n` parameter does not exceed the bounds of the `s` array
        eprintln!("Buffer overflow detected in inttobits");
        return;
    }
    s[n as usize] = 0;
    let mut c = c;
    let mut n = n;
    while n > 0 {
        s[(n - 1) as usize] = (c % 2) as u8 + b'0';
        c >>= 1;
        n -= 1;
    }
}

const TEST: &str = "this is an example for huffman encoding";

fn main() {
    unsafe {
        let r: *mut *mut Huffcode; // Modified: Changed type to match the return type of create_huffman_codes
        let mut i: usize;
        let mut strbit: [u8; MAXBITSPERCODE] = [0; MAXBITSPERCODE];
        let mut p: *const u8 = TEST.as_ptr();
        let mut freqs: [i64; BYTES] = [0; BYTES];

        while *p != 0 {
            freqs[*p as usize] += 1;
            p = p.offset(1);
        }

        r = create_huffman_codes(freqs.as_mut_ptr());
        if r.is_null() { // Modified: Ensure create_huffman_codes does not return a null pointer
            eprintln!("Failed to create Huffman codes");
            return;
        }

        for i in 0..BYTES {
            if !(*r.offset(i as isize)).is_null() {
                inttobits((**r.offset(i as isize)).code, (**r.offset(i as isize)).nbits, &mut strbit);
                println!("{} ({}) {}", i as u8 as char, (**r.offset(i as isize)).code, std::str::from_utf8(&strbit).unwrap());
            }
        }

        free_huffman_codes(r);
    }
}