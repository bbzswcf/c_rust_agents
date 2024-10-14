use std::io;

type SetT = u32; // probably 32 bits; change according to need

fn show_set(x: SetT, name: &str) {
    print!("{} is:", name);
    // Modified: Limited the loop to the number of bits in `SetT` (32 for `u32`)
    for i in 0..32 {
        if (1 << i) > x {
            break;
        }
        if x & (1 << i) != 0 {
            print!(" {}", i);
        }
    }
    println!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut a: SetT = 0; // empty set
    for i in (0..10).step_by(3) { // add 0 3 6 9 to set a
        a |= 1 << i;
    }
    show_set(a, "a");

    for i in 0..5 {
        println!("\t{} is{} in set a", i, if a & (1 << i) != 0 { "" } else { " not" });
    }

    let mut b = a;
    b |= 1 << 5;
    b |= 1 << 10; // b is a plus 5, 10
    b &= !(1 << 0); // sans 0
    show_set(b, "b");

    show_set(a | b, "union(a, b)");
    let c = a & b;
    show_set(c, "c = common(a, b)");
    show_set(a & !b, "a - b"); // diff, not arithmetic minus
    show_set(b & !a, "b - a");

    // Modified: Corrected the subset check for `b`
    println!("b is{} a subset of a", if (b & a) == b { "" } else { " not" }); // Checks if all bits set in `b` are also set in `a`

    // Modified: Corrected the subset check for `c`
    println!("c is{} a subset of a", if (c & a) == c { "" } else { " not" }); // Checks if all bits set in `c` are also set in `a`

    // Modified: Corrected the final equality check
    println!(
        "union(a, b) - common(a, b) {} union(a - b, b - a)",
        if ((a | b) & !(a & b)) == ((a & !b) | (b & !a)) {
            "equals"
        } else {
            "does not equal"
        }
    ); // Correctly checks if the symmetric difference of `a` and `b` is equal to the union of their differences

    Ok(())
}