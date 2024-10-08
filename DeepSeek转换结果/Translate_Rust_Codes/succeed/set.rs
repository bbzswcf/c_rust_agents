type SetT = u32; // 32; 

fn show_set(x: SetT, name: &str) {
    print!("{} is:", name);
    for i in 0.. {
        if (1_u32 << i) > x {
            break;
        }
        if x & (1_u32 << i) != 0 {
            print!(" {}", i);
        }
    }
    println!();
}

fn main() {
    let mut a: SetT = 0; // 
    for i in (0..10).step_by(3) { //  0 3 6 9  a
        a |= 1_u32 << i;
    }
    show_set(a, "a");

    for i in 0..5 {
        println!("\t{}{} in set a", i, if a & (1_u32 << i) != 0 { "" } else { " not" });
    }

    let mut b = a;
    b |= 1_u32 << 5;
    b |= 1_u32 << 10; // b  a  5, 10
    b &= !(1_u32 << 0); //  0
    show_set(b, "b");

    show_set(a | b, "union(a, b)");
    let c = a & b;
    show_set(c, "c = common(a, b)");
    show_set(a & !b, "a - b"); // , 
    show_set(b & !a, "b - a");
    println!("b is{} a subset of a", if b & !a == 0 { "" } else { " not" });
    println!("c is{} a subset of a", if c & !a == 0 { "" } else { " not" });

    println!(
        "union(a, b) - common(a, b) {} union(a - b, b - a)",
        if (a | b) & !(a & b) == (a & !b) | (b & !a) {
            "equals"
        } else {
            "does not equal"
        }
    );
}