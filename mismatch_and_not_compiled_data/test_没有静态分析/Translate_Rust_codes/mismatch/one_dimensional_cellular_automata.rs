fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> usize {
    let mut diff = 0;
    let trans = "__#_##_";

    for i in 0..len {
        // Ensure boundary checks to prevent out-of-bounds access
        let index = (if i > 0 && i < len - 1 && cell[i - 1] != '_' { 1 } else { 0 }) * 4
            + (if cell[i] != '_' { 1 } else { 0 }) * 2
            + (if i < len - 1 && cell[i + 1] != '_' { 1 } else { 0 });
        // Handle `None` case by providing a default value `'_'`
        backup[i] = trans.chars().nth(index).unwrap_or('_');
        // Ensure character comparison is done correctly
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
    // Ensure newline character is handled correctly
    let mut c = "_###_##_#_#_#_#__#__".chars().collect::<Vec<char>>();
    let mut b = "____________________".chars().collect::<Vec<char>>();

    // Create temporary variables to store the length calculations
    let c_len = c.len();
    let b_len = b.len();

    // Create temporary slices to avoid conflicting borrows
    let c_slice = &mut c[1..c_len - 1];
    let b_slice = &mut b[1..b_len - 1];

    // Calculate the length of the slices directly
    let slice_len = c_slice.len();
    while evolve(c_slice, b_slice, slice_len) > 0 {
        // The mutable borrows are still in use here
    }

    // The mutable borrows are dropped here
    print!("{}", c.iter().collect::<String>());
}