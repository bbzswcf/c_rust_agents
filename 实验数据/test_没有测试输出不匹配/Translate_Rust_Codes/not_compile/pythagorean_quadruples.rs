const N: usize = 2200;
// Modified: Ensure the calculation does not overflow by using u128 and then casting back to usize
const N2: usize = ((2200_u128 * 2200) * 2) as usize;

fn main() {
    // Modified: The vector `r` is not modified after initialization, so it does not need to be mutable
    let r = vec![0; N + 1];
    // Modified: The vector `ab` is not modified after initialization, so it does not need to be mutable
    let ab = vec![0; N2 + 1];

    for a in 1..=N {
        let a2 = a * a;
        for b in a..=N {
            if a2 + b * b <= N2 {
                ab[a2 + b * b] = 1;
            }
        }
    }

    for c in 1..=N {
        // Modified: The variable `s1` is only assigned once and not modified within the loop, so it does not need to be mutable
        let s1 = 3;
        // Modified: The variable `s2` is only assigned once and not modified within the loop, so it does not need to be mutable
        let s2 = 5;
        for d in (c + 1)..=N {
            if s1 <= N2 && ab[s1] == 1 {
                r[d] = 1;
            }
            // Modified: Removed redundant check for s1 > N2
            // if s1 > N2 {
            //     break;
            // }
            // Note: s1 and s2 are not mutable, so their values are not updated within the loop
        }
    }

    for d in 1..=N {
        if r[d] == 0 {
            print!("{} ", d);
        }
    }
    println!();
}