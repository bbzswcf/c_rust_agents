use std::ptr;
use std::mem::{self, MaybeUninit};

fn swap<T>(va: &mut T, vb: &mut T) {
    unsafe {
        // Use MaybeUninit to safely create uninitialized memory
        let mut temp: MaybeUninit<T> = MaybeUninit::uninit();
        ptr::copy_nonoverlapping(va, temp.as_mut_ptr(), 1);
        ptr::copy_nonoverlapping(vb, va, 1);
        ptr::copy_nonoverlapping(temp.as_ptr(), vb, 1);
        // No need to forget temp, it will be dropped automatically
    }
}

macro_rules! swap {
    ($x:expr, $y:expr) => {
        {
            // Swap the values directly, not the references
            let temp = *$x;
            *$x = *$y;
            *$y = temp;
        }
    };
}

struct Test {
    a: i32,
    b: i32,
    c: i32,
}

fn main() {
    let mut t = Test { a: 1, b: 2, c: 3 };
    let mut h = Test { a: 4, b: 5, c: 6 };
    let mut alfa = 0.45;
    let mut omega = 9.98;

    // Separate borrows in time to avoid mutable borrow conflicts
    {
        let mut pt = &mut t;
        println!("{} {} {}", t.a, t.b, t.c); // Print t before borrowing h mutably
    }
    {
        let mut th = &mut h;
        println!("{} {} {}", h.a, h.b, h.c); // Print h before swapping
    }
    swap!(&mut t.a, &mut h.a); // Swap the fields of the structs
    println!("{} {} {}", t.a, t.b, t.c);
    println!("{} {} {}", h.a, h.b, h.c);

    // Separate borrows in time to avoid mutable borrow conflicts
    println!("{}", alfa); // Print alfa before swapping
    swap!(&mut alfa, &mut omega); // Swap the values directly
    println!("{}", alfa);

    // Separate borrows in time to avoid mutable borrow conflicts
    {
        let pt_ref = &t.a;
        println!("{}", pt_ref); // Print pt_ref before borrowing pt.a mutably
    }
    {
        let mut_pt_ref = &mut t.a;
        let mut_th_ref = &mut h.a;
        println!("{}", mut_pt_ref); // Print mut_pt_ref before swapping
        swap!(mut_pt_ref, mut_th_ref); // Swap the fields of the structs
    }
    println!("{}", t.a);
}