const TRANS: &str = "___#_##_";

fn evolve(cell: &mut [char], backup: &mut [char], len: usize) -> i32 {
    let mut diff = 0;

    for i in 0..len {
        // 修改: 确保 index 在 TRANS 的有效范围内
        let index = (if i > 0 { cell[i - 1] != '_' } else { false } as usize) * 4
            + (cell[i] != '_') as usize * 2
            + (if i < len - 1 { cell[i + 1] != '_' } else { false } as usize);
        // 修改: 确保 index 在 TRANS 的有效范围内
        if index >= TRANS.len() {
            eprintln!("Invalid index: {}", index);
            backup[i] = '_';
        } else {
            backup[i] = TRANS.chars().nth(index).unwrap();
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
    // 修改: 确保 c 和 b 的长度一致
    let mut c = "_###_##_#_#_#_#__#__".chars().collect::<Vec<char>>();
    let mut b = vec!['_'; c.len()];

    loop {
        // 修改: 使用 println! 替代 print!
        println!("{}", c.iter().collect::<String>());
        // 修改: 确保 c_slice 和 b_slice 的长度一致
        let len = c.len() - 2;
        let c_slice = &mut c[1..c.len() - 1];
        let b_slice = &mut b[1..b.len() - 1];
        if evolve(c_slice, b_slice, len) == 0 {
            break;
        }
    }
}