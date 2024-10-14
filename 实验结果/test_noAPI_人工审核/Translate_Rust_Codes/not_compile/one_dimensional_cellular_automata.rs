use std::ptr;

const TRANS: &str = "___#_##_";

fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> i32 {
    let mut diff = 0;

    for i in 0..len {
        let left = if i > 0 && cell[i - 1] != '_' { 1 } else { 0 };
        let self_val = if cell[i] != '_' { 1 } else { 0 };
        let right = if i < len - 1 && cell[i + 1] != '_' { 1 } else { 0 };

        let index = left * 4 + self_val * 2 + right;
        backup[i] = TRANS.chars().nth(index).unwrap();
        if backup[i] != cell[i] {
            diff += 1;
        }
    }

    unsafe {
        ptr::copy_nonoverlapping(backup.as_ptr(), cell.as_mut_ptr(), len);
    }

    diff
}

fn main() {
    let mut c = "_###_##_#_#_#_#__#__\n".chars().collect::<Vec<char>>();
    let mut b = "____________________\n".chars().collect::<Vec<char>>();

    // Store the lengths in local variables to avoid mutable and immutable borrow conflicts
    let len_c = c.len() - 3;
    let len_b = b.len() - 2;

    // Create a local variable to store the immutable reference to `c`
    let c_immutable = &c;

    // Isolate the mutable borrow of `c` to `c_slice` in a block scope
    while {
        {
            let mut c_slice = &mut c[1..c.len() - 2];
            evolve(c_slice, &mut b[1..len_b], len_c) != 0
        } // Modified: Isolated the mutable borrow of `c` to `c_slice` to avoid conflict with immutable borrow in `print!`
    } {
        // Use the immutable reference to `c` to avoid conflicts
        print!("{}", c_immutable.iter().skip(1).collect::<String>());
    }
}