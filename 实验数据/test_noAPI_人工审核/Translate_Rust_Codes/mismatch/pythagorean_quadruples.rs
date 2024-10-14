use std::mem;

const N: usize = 2200;
const N2: usize = 2200 * 2200 * 2;

fn main() {
    // Removed: The check is always false because N + 1 cannot exceed usize::MAX
    let mut r = vec![0; N + 1];

    // Removed: The check is always false because N2 / 100 + 1 cannot exceed usize::MAX
    let mut ab = vec![0; N2 / 100 + 1];

    for a in 1..=N {
        // Modified: Handle overflow case more gracefully
        if let Some(a2) = (a as u64).checked_mul(a as u64) {
            for b in a..=N {
                // Modified: Handle overflow case more gracefully
                if let Some(b2) = (b as u64).checked_mul(b as u64) {
                    if a2 + b2 <= N2 as u64 {
                        // Modified: Ensure the index calculations do not result in out-of-bounds access
                        let index = (a2 + b2) as usize / 100;
                        if index < ab.len() {
                            ab[index] = 1;
                        }
                    }
                } else {
                    continue;
                }
            }
        } else {
            continue;
        }
    }

    for c in 1..=N {
        let mut s1 = 3;
        let mut s2 = 5;
        for d in (c + 1)..=N {
            // Modified: Ensure the index calculations do not result in out-of-bounds access
            if s1 < ab.len() && s1 / 100 < ab.len() && ab[s1 / 100] == 1 {
                r[d] = 1;
            }
            // Modified: Ensure the loop terminates correctly when arithmetic operations overflow
            if let Some(new_s1) = s1.checked_add(s2) {
                s1 = new_s1;
            } else {
                break;
            }
            if let Some(new_s2) = s2.checked_add(2) {
                s2 = new_s2;
            } else {
                break;
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