fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> usize {
    let mut diff = 0;

    // Precompute the values to avoid redundant array access
    let rules: [char; 8] = ['_', '_', '_', '#', '_', '#', '#', '_'];

    // Borrow `cell` for reading and `backup` for writing
    for i in 0..len {
        // Ensure index calculations do not result in out-of-bounds access
        let left = if i > 0 && cell[i - 1] != '_' { 1 } else { 0 };
        let center = if cell[i] != '_' { 1 } else { 0 };
        let right = if i < len - 1 && cell[i + 1] != '_' { 1 } else { 0 };
        let index = left * 4 + center * 2 + right;
        backup[i] = rules[index];
        if backup[i] != cell[i] {
            diff += 1;
        }
    }

    // Borrow `backup` for reading and `cell` for writing
    for i in 0..len {
        cell[i] = backup[i];
    }

    diff
}

fn main() {
    let mut c = "_###_##_#_#_#_#__#__\n".chars().collect::<Vec<char>>();
    let mut b = "____________________\n".chars().collect::<Vec<char>>();

    // Calculate the length directly where needed to avoid redundant variables
    let len_c = c.len() - 3;

    // Store the slices in temporary variables to avoid recalculating them each time
    let mut c_slice = &mut c[1..c.len() - 2];
    let mut b_slice = &mut b[1..b.len() - 2];

    // Ensure slices are not borrowed with different mutabilities within the same scope
    while evolve(c_slice, b_slice, len_c) != 0 {
        // Use `&c` directly where needed to avoid unnecessary variables
        print!("{}", c.iter().collect::<String>());
    }
}