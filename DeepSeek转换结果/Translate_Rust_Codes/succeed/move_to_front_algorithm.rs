use std::cmp::Ordering;
use std::mem;
use std::ptr;
use std::slice;

const MAX_SIZE: usize = 100;

fn move_to_front(s: &mut [char], c: char) -> usize {
    if let Some(q) = s.iter().position(|&x| x == c) {
        let mut new_s = vec![c];
        new_s.extend(s.iter().take(q));
        new_s.extend(s.iter().skip(q + 1));
        s.copy_from_slice(&new_s);
        q
    } else {
        0
    }
}

fn decode(pass: &[usize], size: usize, sym: &mut [char]) {
    let mut table: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for i in 0..size {
        let c = table[pass[i]];
        let index = move_to_front(&mut table, c);
        if pass[i] != index {
            print!("there is an error");
        }
        sym[i] = c;
    }
    sym[size] = '\0';
}

fn encode(sym: &[char], size: usize, pass: &mut [usize]) {
    let mut table: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for i in 0..size {
        let c = sym[i];
        pass[i] = move_to_front(&mut table, c);
    }
}

fn check(sym: &[char], size: usize, pass: &[usize]) -> bool {
    let mut pass2 = vec![0; size];
    let mut sym2 = vec!['\0'; size + 1];
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
    if sym.cmp(&sym2[..size]) != Ordering::Equal {
        val = false;
    }

    val
}

fn main() {
    let sym = vec![
        "broood".chars().collect::<Vec<char>>(),
        "bananaaa".chars().collect::<Vec<char>>(),
        "hiphophiphop".chars().collect::<Vec<char>>(),
    ];
    let mut pass = vec![0; MAX_SIZE];
    for i in 0..3 {
        let len = sym[i].len();
        encode(&sym[i], len, &mut pass);
        print!("{} : [", sym[i].iter().collect::<String>());
        for j in 0..len {
            print!("{} ", pass[j]);
        }
        println!("]");
        if check(&sym[i], len, &pass) {
            println!("Correct :)");
        } else {
            println!("Incorrect :(");
        }
    }
}