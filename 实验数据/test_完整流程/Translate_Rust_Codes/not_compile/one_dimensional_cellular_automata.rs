fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> usize {
    let trans = "__#_##_";
    let mut diff = 0;

    for i in 0..len {
        let index = (if i > 0 && cell[i - 1] != '_' { 1 } else { 0 }) * 4
            + (if cell[i] != '_' { 1 } else { 0 }) * 2
            + (if i < len - 1 && cell[i + 1] != '_' { 1 } else { 0 });
        // Ensure index is within bounds of `trans`
        if index < trans.len() {
            backup[i] = trans.chars().nth(index).unwrap();
        } else {
            // Handle the out-of-bounds case appropriately
            backup[i] = '_'; // Default to '_' if index is out of bounds
        }
        if backup[i] != cell[i] {
            diff += 1;
        }
    }

    for i in 0..len {
        cell[i] = backup[i];
    }

    diff
}

fn main() {
    let mut c = "_###_##_#_#_#_#__#__\n".chars().collect::<Vec<char>>();
    let mut b = "____________________\n".chars().collect::<Vec<char>>();

    // Store the length calculation in a local variable before the mutable borrow
    let len = c.len() - 3;

    // Print the state of `c` before creating the mutable borrow `c_slice`
    print!("{}", c.iter().collect::<String>());

    {
        // Create temporary slices to hold the mutable borrows
        let c_slice = &mut c[1..c.len() - 2]; // Removed `mut` keyword
        let b_slice = &mut b[1..b.len() - 2]; // Removed `mut` keyword

        while evolve(c_slice, b_slice, len) > 0 {
            // Print the state of `c` before the next mutable borrow
            print!("{}", c.iter().collect::<String>());
        }
    }

    // Print the state of `c` after the mutable borrow scope ends
    print!("{}", c.iter().collect::<String>());
}