use std::ptr;

const TRANS: &str = "___#_##_";

fn v(cell: &[char], i: i32) -> bool {
    // Modified: Added bounds checking to avoid out-of-bounds access
    if i < 0 || i >= cell.len() as i32 {
        return false;
    }
    cell[i as usize] != '_'
}

fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> i32 {
    let mut diff = 0;

    for i in 0..len {
        let index = (v(cell, (i as i32) - 1) as usize) * 4
            + (v(cell, i as i32) as usize) * 2
            + (v(cell, (i as i32) + 1) as usize);
        backup[i] = TRANS.chars().nth(index).unwrap();
        if backup[i] != cell[i] {
            diff += 1;
        }
    }

    // Modified: Replaced unsafe block with safe Rust method
    cell.copy_from_slice(backup);

    diff
}

fn main() {
    let mut c = "_###_##_#_#_#_#__#__\n".chars().collect::<Vec<char>>();
    let mut b = "____________________\n".chars().collect::<Vec<char>>();

    loop {
        println!("{}", c.iter().skip(1).collect::<String>());
        
        // Store the lengths of the slices in local variables to avoid conflicts
        let c_len = c.len();
        let b_len = b.len();
        
        // Use split_at_mut to create non-overlapping mutable slices
        let (c_left, c_right) = c.split_at_mut(c_len - 2);
        let (b_left, b_right) = b.split_at_mut(b_len - 2);
        
        // Modified: Created a temporary slice to avoid mutable borrow conflict
        let temp_c_right = &mut c_right[1..];
        let temp_b_right = &mut b_right[1..];
        
        if evolve(temp_c_right, temp_b_right, temp_c_right.len()) == 0 {
            break;
        }
    }
}