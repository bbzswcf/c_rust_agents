use std::cmp::Ordering;

const MAX_SIZE: usize = 100;

fn move_to_front(s: &mut String, c: char) -> usize {
    // Correctly reorder the string `s` after finding the character `c`
    if let Some(index) = s.find(c) {
        let mut chars: Vec<char> = s.chars().collect();
        chars.remove(index);
        chars.insert(0, c);
        *s = chars.iter().collect();
        index
    } else {
        0
    }
}

fn decode(pass: &[usize], size: usize, sym: &mut [char]) {
    let mut table = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in 0..size {
        // Ensure that the index `pass[i]` is within the bounds of the `table` string
        if pass[i] >= table.len() {
            println!("Index out of bounds error");
            return;
        }
        let c = table.chars().nth(pass[i]).unwrap();
        let index = move_to_front(&mut table, c);
        if pass[i] != index {
            println!("there is an error");
        }
        sym[i] = c;
    }
}

fn encode(sym: &str, size: usize, pass: &mut [usize]) {
    let mut table = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in 0..size {
        // Ensure that `i` is a valid index within the bounds of the `sym` string
        if i >= sym.len() {
            println!("Index out of bounds error");
            return;
        }
        let c = sym.chars().nth(i).unwrap();
        pass[i] = move_to_front(&mut table, c);
    }
}

fn check(sym: &str, size: usize, pass: &[usize]) -> bool {
    let mut pass2 = vec![0; size];
    let mut sym2 = vec!['\0'; size];
    let mut val = true;

    encode(sym, size, &mut pass2);
    let mut i = 0;
    while i < size && pass[i] == pass2[i] {
        i += 1;
    }
    if i != size {
        val = false;
    }

    decode(pass, size, &mut sym2);
    if sym.cmp(&sym2.iter().collect::<String>()) != Ordering::Equal {
        val = false;
    }

    val
}

fn main() {
    let sym = vec!["broood", "bananaaa", "hiphophiphop"];
    let mut pass = vec![0; MAX_SIZE];
    for i in 0..3 {
        // Ensure that `i` is a valid index within the bounds of the `sym` vector
        if i >= sym.len() {
            println!("Index out of bounds error");
            return;
        }
        let len = sym[i].len();
        // Ensure that `len` is a valid length for the `pass` vector
        if len > pass.len() {
            println!("Length exceeds pass vector size");
            return;
        }
        encode(sym[i], len, &mut pass);
        print!("{} : [", sym[i]);
        for j in 0..len {
            print!("{} ", pass[j]);
        }
        println!("]");
        if check(sym[i], len, &pass) {
            println!("Correct :)");
        } else {
            println!("Incorrect :(");
        }
    }
}