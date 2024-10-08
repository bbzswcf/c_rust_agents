use std::collections::HashSet;

const N: usize = 2200;
const N2: usize = 2200 * 2200 * 2; // Modified: Ensure usize can accommodate the value

fn main() {
    // Modified: Use HashSet to avoid unnecessary memory allocation and initialization
    let mut ab = HashSet::new();

    for a in 1..=N {
        let a2 = a * a;
        for b in a..=N {
            let index = a2 + b * b;
            // Modified: Ensure the index does not exceed the bounds of the vector
            if index <= N2 {
                ab.insert(index);
            }
        }
    }

    // Modified: Use HashSet to avoid unnecessary memory allocation and initialization
    let mut r = HashSet::new();

    let mut s = 3;
    for c in 1..=N {
        let mut s1 = s;
        s += 2;
        let mut s2 = s;
        for d in (c + 1)..=N {
            // Modified: Ensure the index does not exceed the bounds of the vector
            if s1 <= N2 && ab.contains(&s1) {
                r.insert(d);
            }
            s1 += s2;
            s2 += 2;
        }
    }

    for d in 1..=N {
        if !r.contains(&d) {
            print!("{} ", d);
        }
    }
    println!();
}