use std::mem;

const N: usize = 2200;
const N2: usize = 2200 * 2200 * 2;

fn main() {
    // Removed unused variable declarations and unnecessary type annotations
    let mut s = 3;
    let mut r = vec![0; N + 1];
    let mut ab = vec![0; N2 + 1];

    for a in 1..=N {
        let a2 = a * a;
        for b in a..=N {
            // Modified: Ensure that the index `a2 + b * b` does not exceed the bounds of the vector `ab`
            // and handle potential overflow in index calculation
            if let Some(index) = a2.checked_add(b.pow(2)) {
                if index <= N2 {
                    ab[index] = 1;
                }
            }
        }
    }

    for c in 1..=N {
        let mut s1 = s;
        s += 2;
        let mut s2 = s;
        for d in (c + 1)..=N {
            // Modified: Ensure that the index `s1` does not exceed the bounds of the vector `ab`
            if s1 <= N2 && ab[s1] != 0 {
                r[d] = 1;
            }
            // Modified: Ensure that the index calculation `s1 += s2` does not overflow
            if let Some(new_s1) = s1.checked_add(s2) {
                s1 = new_s1;
            } else {
                break; // or handle overflow appropriately
            }
            // Modified: Ensure that the index calculation `s2 += 2` does not overflow
            if let Some(new_s2) = s2.checked_add(2) {
                s2 = new_s2;
            } else {
                break; // or handle overflow appropriately
            }
        }
    }

    for d in 1..=N {
        if r[d] == 0 {
            print!("{} ", d);
        }
    }
    println!();
}