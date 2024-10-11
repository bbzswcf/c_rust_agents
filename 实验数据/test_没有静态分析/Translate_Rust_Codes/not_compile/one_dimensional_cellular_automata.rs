fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> usize {
    let mut diff = 0;

    for i in 0..len {
        let left = if i > 0 && cell[i - 1] != '_' { 1 } else { 0 };
        let self_val = if cell[i] != '_' { 1 } else { 0 };
        let right = if i < len - 1 && cell[i + 1] != '_' { 1 } else { 0 };

        backup[i] = ['_', '_', '_', '#', '_', '#', '#', '_'][left * 4 + self_val * 2 + right];
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

    let len = c.len() - 3;
    while {
        // Temporarily store the result of the mutable borrow
        let result = evolve(&mut c[1..c.len() - 2], &mut b[1..b.len() - 2], len);
        // Drop the mutable borrow before the immutable borrow
        result != 0
    } {
        // Ensure mutable borrow of `c` is dropped before immutable borrow
        print!("{}", c.iter().collect::<String>());
    }
}