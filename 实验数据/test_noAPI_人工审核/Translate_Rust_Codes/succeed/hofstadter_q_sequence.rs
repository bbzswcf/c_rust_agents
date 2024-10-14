use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

const N: usize = 100000;

fn main() {
    unsafe {
        // Ensure the layout is computed only once
        let layout = Layout::array::<i32>(N + 1).unwrap();
        let q_ptr = alloc(layout) as *mut i32;

        // Ensure pointer arithmetic does not go out of bounds
        let q = q_ptr.offset(1);

        ptr::write(q, 1);
        ptr::write(q.offset(1), 1);

        for i in 2..N {
            let val = ptr::read(q.offset((i - ptr::read(q.offset((i - 1) as isize)) as usize) as isize))
                + ptr::read(q.offset((i - ptr::read(q.offset((i - 2) as isize)) as usize) as isize));
            ptr::write(q.offset(i as isize), val);
        }

        for i in 0..10 {
            print!("{}", ptr::read(q.offset(i as isize)));
            if i == 9 {
                println!();
            } else {
                print!(" ");
            }
        }

        println!("{}", ptr::read(q.offset(999 as isize)));

        let mut flip = 0;
        for i in 0..(N - 1) {
            if ptr::read(q.offset(i as isize)) > ptr::read(q.offset((i + 1) as isize)) {
                flip += 1;
            }
        }

        println!("flips: {}", flip);

        // Ensure memory is correctly deallocated
        dealloc(q_ptr as *mut u8, layout);
    }
}