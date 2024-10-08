use std::mem;
use std::alloc::{self, Layout};

type Ham = u64;

static mut ALLOC: usize = 0;
static mut N: usize = 1;
static mut Q: *mut Ham = std::ptr::null_mut();

unsafe fn qpush(h: Ham) {
    let mut i: usize;
    let mut j: usize;

    // Ensure ALLOC and N are initialized correctly before this point
    if ALLOC <= N {
        let old_size = ALLOC * mem::size_of::<Ham>(); // Calculate old allocation size
        ALLOC = if ALLOC > 0 { ALLOC.checked_mul(2).unwrap_or(usize::MAX) } else { 16 }; // Modified: Use unwrap_or(usize::MAX) to handle overflow
        Q = realloc(Q, old_size, ALLOC); // Pass old allocation size to realloc
    }

    i = N;
    N += 1;
    while {
        j = i / 2;
        j > 0 && *Q.offset(j as isize) > h
    } {
        if !Q.is_null() { // Ensure Q is not null before dereferencing
            *Q.offset(i as isize) = *Q.offset(j as isize);
        }
        i = j;
    }
    if !Q.is_null() { // Ensure Q is not null before dereferencing
        *Q.offset(i as isize) = h;
    }
}

unsafe fn qpop() -> Ham {
    let mut i: usize;
    let mut j: usize;
    let mut r: Ham;
    let mut t: Ham;

    loop {
        if Q.is_null() { // Ensure Q is not null before dereferencing
            return 0; // Return a default value if Q is null
        }
        r = *Q.offset(1);
        if N <= 1 || r != *Q.offset(1) {
            break;
        }

        i = 1;
        t = *Q.offset((N - 1) as isize);
        N -= 1;
        
        // Evaluate the assignment separately before the while loop condition
        j = i * 2;
        while j < N {
            if j + 1 < N && *Q.offset(j as isize) > *Q.offset((j + 1) as isize) {
                j += 1;
            }
            if t <= *Q.offset(j as isize) {
                break;
            }
            *Q.offset(i as isize) = *Q.offset(j as isize);
            i = j;
            j = i * 2; // Update j for the next iteration
        }
        *Q.offset(i as isize) = t;
    }

    r
}

unsafe fn realloc(ptr: *mut Ham, old_size: usize, new_size: usize) -> *mut Ham {
    // Ensure ptr is correctly cast to *mut u8 and new_size is calculated correctly
    let layout = Layout::from_size_align(old_size, mem::align_of::<Ham>()).unwrap(); // Modified: Use from_size_align instead of unchecked
    let new_ptr = alloc::realloc(ptr as *mut u8, layout, new_size * mem::size_of::<Ham>()); // Modified: Correct calculation of new_size
    new_ptr as *mut Ham
}

fn main() {
    unsafe {
        let mut i: usize = 1;
        let mut h: Ham;

        // Ensure i is within the valid range for u64
        qpush(i as Ham); // Ensure `i` is correctly cast to `Ham`
        while i <= 1691 {
            h = qpop();
            qpush((h * 2) as Ham); // Ensure the result of `h * 2` is correctly typed as `Ham`
            qpush((h * 3) as Ham); // Ensure the result of `h * 3` is correctly typed as `Ham`
            qpush((h * 5) as Ham); // Ensure the result of `h * 5` is correctly typed as `Ham`

            if i <= 20 || i == 1691 {
                println!("{:6}: {}", i, h);
            }
            i += 1;
        }

        // Ensure Q is correctly cast to *mut u8 and Layout is calculated correctly
        if !Q.is_null() { // Ensure Q is not null before deallocating
            let layout = Layout::from_size_align(ALLOC * mem::size_of::<Ham>(), mem::align_of::<Ham>()).unwrap(); // Modified: Use from_size_align instead of unchecked
            alloc::dealloc(Q as *mut u8, layout);
        }
    }
}