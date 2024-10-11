fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> usize {
    let mut diff = 0;

    for i in 0..len {
        let index = (if i > 0 && cell[i - 1] != '_' { 1 } else { 0 }) * 4
            + (if cell[i] != '_' { 1 } else { 0 }) * 2
            + (if i < len - 1 && cell[i + 1] != '_' { 1 } else { 0 });
        backup[i] = ['_', '_', '_', '#', '_', '#', '#', '_'][index];
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

    while evolve(&mut c[1..c.len() - 2], &mut b[1..b.len() - 2], c.len() - 3) != 0 {
        print!("{}", c.iter().collect::<String>());
    }
}