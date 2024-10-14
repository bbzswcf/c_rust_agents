// Added: Include the `libc` crate in your project
extern crate libc;

// Added: Import the `TryInto` trait to use the `try_into` method
use std::convert::TryInto;
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
    h: *mut i32,
    n: i32,
    s: i32,
    cs: i32,
    f: *mut i64,
}

// Modified: Encapsulate unsafe operations within safe abstractions
fn heap_create(s: i32, f: *mut i64) -> Result<*mut Huffheap, &'static str> {
    let heap = unsafe { libc::malloc(mem::size_of::<Huffheap>()) as *mut Huffheap };
    if heap.is_null() {
        return Err("Failed to allocate memory for Huffheap");
    }
    unsafe {
        (*heap).h = libc::malloc((s as usize) * mem::size_of::<i32>()) as *mut i32;
        if (*heap).h.is_null() {
            libc::free(heap as *mut libc::c_void);
            return Err("Failed to allocate memory for heap array");
        }
        (*heap).s = s;
        (*heap).cs = s;
        (*heap).n = 0;
        (*heap).f = f;
    }
    Ok(heap)
}

// Modified: Encapsulate unsafe operations within safe abstractions
fn heap_destroy(heap: *mut Huffheap) {
    unsafe {
        libc::free((*heap).h as *mut libc::c_void);
        libc::free(heap as *mut libc::c_void);
    }
}

// Modified: Encapsulate unsafe operations within safe abstractions
fn heap_sort(heap: *mut Huffheap) {
    unsafe {
        let mut i = 1;
        let mut j = 2;
        let a = (*heap).h;

        while i < (*heap).n {
            // Modified: Convert `i32` values to `isize` using `try_into().unwrap()`
            if *(*heap).f.offset((*a.offset((i - 1).try_into().unwrap())) as isize) >= *(*heap).f.offset((*a.offset(i.try_into().unwrap())) as isize) {
                i = j;
                j += 1;
            } else {
                ptr::swap(a.offset((i - 1).try_into().unwrap()), a.offset(i.try_into().unwrap()));
                i -= 1;
                i = if i == 0 { j } else { i };
                j += 1;
            }
        }
    }
}

// Modified: Encapsulate unsafe operations within safe abstractions
fn heap_add(heap: *mut Huffheap, c: i32) -> Result<(), &'static str> {
    unsafe {
        // Modified: Ensure that the addition `(*heap).n + 1` does not overflow
        if (*heap).n.checked_add(1).unwrap_or(i32::MAX) > (*heap).s {
            (*heap).h = libc::realloc(
                (*heap).h as *mut libc::c_void,
                ((*heap).s + (*heap).cs) as usize * mem::size_of::<i32>(),
            ) as *mut i32;
            if (*heap).h.is_null() {
                return Err("Failed to reallocate memory for heap array");
            }
            (*heap).s += (*heap).cs;
        }
        *(*heap).h.offset((*heap).n as isize) = c;
        (*heap).n += 1;
        heap_sort(heap);
    }
    Ok(())
}

// Modified: Encapsulate unsafe operations within safe abstractions
fn heap_remove(heap: *mut Huffheap) -> Result<i32, &'static str> {
    unsafe {
        if (*heap).n > 0 {
            // Modified: Ensure that the subtraction `(*heap).n -= 1` does not underflow
            if let Some(new_n) = (*heap).n.checked_sub(1) {
                (*heap).n = new_n;
                Ok(*(*heap).h.offset((*heap).n as isize))
            } else {
                Err("Heap underflow")
            }
        } else {
            Err("Heap is empty")
        }
    }
}

// Modified: Encapsulate unsafe operations within safe abstractions
fn create_huffman_codes(freqs: *mut i64) -> Result<*mut *mut Huffcode, &'static str> {
    let mut codes: *mut *mut Huffcode = unsafe { libc::malloc(BYTES * mem::size_of::<*mut Huffcode>()) as *mut *mut Huffcode };
    if codes.is_null() {
        return Err("Failed to allocate memory for Huffcode array");
    }
    let mut heap: *mut Huffheap = heap_create(BYTES as i32 * 2, freqs)?;
    let mut efreqs: [i64; BYTES * 2] = [0; BYTES * 2];
    let mut preds: [i32; BYTES * 2] = [0; BYTES * 2];
    let mut extf = BYTES as i32;
    let mut r1;
    let mut r2;

    unsafe {
        ptr::copy_nonoverlapping(freqs, efreqs.as_mut_ptr(), BYTES);
        ptr::write_bytes(efreqs.as_mut_ptr().offset(BYTES as isize), 0, BYTES);

        if heap.is_null() {
            libc::free(codes as *mut libc::c_void);
            return Err("Failed to create heap");
        }

        for i in 0..BYTES {
            if *efreqs.as_ptr().offset(i as isize) > 0 {
                heap_add(heap, i as i32)?;
            }
        }

        while (*heap).n > 1 {
            r1 = heap_remove(heap)?;
            r2 = heap_remove(heap)?;
            *efreqs.as_mut_ptr().offset(extf as isize) = *efreqs.as_ptr().offset(r1 as isize) + *efreqs.as_ptr().offset(r2 as isize);
            heap_add(heap, extf)?;
            preds[r1 as usize] = extf;
            preds[r2 as usize] = -extf;
            extf += 1;
        }
        r1 = heap_remove(heap)?;
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
            if (*codes.offset(i as isize)).is_null() {
                free_huffman_codes(codes);
                return Err("Failed to allocate memory for Huffcode");
            }
            (*(*codes.offset(i as isize))).nbits = bn;
            (*(*codes.offset(i as isize))).code = bc;
        }
    }
    Ok(codes)
}

// Modified: Encapsulate unsafe operations within safe abstractions
fn free_huffman_codes(c: *mut *mut Huffcode) {
    unsafe {
        for i in 0..BYTES {
            libc::free(*c.offset(i as isize) as *mut libc::c_void);
        }
        libc::free(c as *mut libc::c_void);
    }
}

const MAXBITSPERCODE: usize = 100;

// Modified: Removed unnecessary `unsafe` block
fn inttobits(c: i32, n: i32, s: &mut [u8]) {
    // Added: Ensure the index is within the bounds of the array
    if n as usize >= s.len() {
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
    let mut r: *mut *mut Huffcode;
    let mut i: usize;
    let mut strbit: [u8; MAXBITSPERCODE] = [0; MAXBITSPERCODE];
    let mut p: *const u8;
    let mut freqs: [i64; BYTES] = [0; BYTES];

    p = TEST.as_ptr();
    unsafe {
        while *p != 0 {
            freqs[*p as usize] += 1;
            p = p.offset(1);
        }
    }

    r = match create_huffman_codes(freqs.as_mut_ptr()) {
        Ok(codes) => codes,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    for i in 0..BYTES {
        // Added: Ensure the pointer dereference is safe by checking for null before dereferencing
        unsafe {
            if !(*r.offset(i as isize)).is_null() {
                inttobits((**r.offset(i as isize)).code, (**r.offset(i as isize)).nbits, &mut strbit);
                println!(
                    "{} ({}) {}",
                    i as u8 as char,
                    (**r.offset(i as isize)).code,
                    std::str::from_utf8_unchecked(&strbit)
                );
            }
        }
    }

    free_huffman_codes(r);
}