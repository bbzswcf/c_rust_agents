use std::ptr;

const TRANS: &str = "___#_##_";

fn v(cell: &[char], i: i32) -> bool {
    // Ensure that the index `i` is within the valid range of the `cell` slice
    if i >= 0 && i < cell.len() as i32 {
        cell[i as usize] != '_'
    } else {
        false // Return false if index is out of bounds
    }
}

fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> i32 {
    let mut diff = 0;

    for i in 0..len {
        // Ensure that the index calculation does not result in an out-of-bounds access
        let left = if i > 0 { v(cell, (i as i32) - 1) } else { false };
        let center = v(cell, i as i32);
        let right = if i < len - 1 { v(cell, (i as i32) + 1) } else { false };

        // Modified: Corrected the index calculation to map boolean values to indices 0, 1, 2, 3, 4, 5, 6, 7
        let index = (left as usize) * 4 + (center as usize) * 2 + (right as usize);
        // Ensure that `index` is within the valid range for `TRANS`
        if index < TRANS.len() {
            backup[i] = TRANS.chars().nth(index).unwrap();
        } else {
            // Modified: Use a default value if `index` is out of bounds
            backup[i] = '_'; // Use a default value if `index` is out of bounds
        }

        if backup[i] != cell[i] {
            diff += 1;
        }
    }

    // Replace the unsafe block with a safe alternative
    cell.copy_from_slice(backup);

    diff
}

fn main() {
    // Modified: Ensure that the string used to collect characters into a vector does not contain any newline characters
    let mut c = "_###_##_#_#_#_#__#__".chars().filter(|&ch| ch != '\n').collect::<Vec<_>>();
    let mut b = "____________________".chars().filter(|&ch| ch != '\n').collect::<Vec<_>>();

    // Store the lengths in local variables to avoid mutable and immutable borrow conflicts
    let c_len = c.len();
    let b_len = b.len();

    // Verify that the slice lengths passed to the `evolve` function are correct
    if c_len >= 4 && b_len >= 4 && c_len == b_len {
        // Modified: Corrected the slice lengths to reflect the intended range
        while evolve(&mut c[1..c_len - 1], &mut b[1..b_len - 1], c_len - 2) != 0 {
            println!("{}", c.iter().skip(1).collect::<String>());
        }
    } else {
        println!("Invalid slice lengths");
    }
}