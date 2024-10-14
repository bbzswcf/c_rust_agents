use std::ptr;

const TRANS: &str = "___#_##_";

fn v(cell: &[char], i: i32) -> bool {
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

    unsafe {
        ptr::copy_nonoverlapping(backup.as_ptr(), cell.as_mut_ptr(), len);
    }

    diff
}

fn main() {
    let mut c = "_###_##_#_#_#_#__#__\n".chars().collect::<Vec<char>>();
    let mut b = "____________________\n".chars().collect::<Vec<char>>();

    loop {
        println!("{}", c.iter().skip(1).collect::<String>());
        if evolve(&mut c[1..c.len() - 2], &mut b[1..b.len() - 2], c.len() - 3) == 0 {
            break;
        }
    }
}