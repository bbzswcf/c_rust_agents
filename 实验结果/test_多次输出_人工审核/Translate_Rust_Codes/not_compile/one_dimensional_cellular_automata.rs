fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> usize {
    let mut diff = 0;
    let trans = ['_', '_', '_', '#', '_', '#', '#', '_'];

    for i in 0..len {
        let left = if i > 0 && cell[i - 1] != '_' { 1 } else { 0 };
        let self_val = if cell[i] != '_' { 1 } else { 0 };
        let right = if i < len - 1 && cell[i + 1] != '_' { 1 } else { 0 };

        backup[i] = trans[left * 4 + self_val * 2 + right];
        if backup[i] != cell[i] {
            diff += 1;
        }
    }

    cell.copy_from_slice(backup);
    diff
}

fn main() {
    let mut c = ['_', '#', '#', '#', '_', '#', '#', '_', '#', '_', '#', '_', '#', '_', '#', '_', '#', '_', '#', '_', '\n'];
    let mut b = ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '\n'];

    while evolve(&mut c[1..c.len() - 2], &mut b[1..b.len() - 2], c.len() - 3) > 0 {
        println!("{}", c.iter().collect::<String>());
    }
}