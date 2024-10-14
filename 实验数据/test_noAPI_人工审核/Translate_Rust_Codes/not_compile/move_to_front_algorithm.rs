use std::ptr;
use std::slice;
use std::str;

const MAX_SIZE: usize = 100;

fn move_to_front(s: &mut [u8], c: u8) -> usize {
    let mut p = Vec::with_capacity(s.len() + 1);
    p.extend_from_slice(s);
    p.push(0);

    // Modified: Added check to handle the case where the element is not found
    if let Some(q) = p.iter().position(|&x| x == c) {
        let shift = q;
        // Modified: Ensure the slice operations do not exceed the bounds of the array
        if shift < s.len() {
            s[1..=shift].copy_from_slice(&p[..shift]);
        }
        s[0] = c;
        shift
    } else {
        // Handle the case where the element is not found
        panic!("Element not found");
    }
}

fn decode(pass: &[usize], size: usize, sym: &mut [u8]) {
    let mut table = b"abcdefghijklmnopqrstuvwxyz".to_vec();
    for i in 0..size {
        // Modified: Ensure the index `pass[i]` does not exceed the bounds of `table`
        if i < pass.len() && pass[i] < table.len() {
            let c = table[pass[i]];
            let index = move_to_front(&mut table, c);
            if pass[i] != index {
                println!("there is an error");
            }
            sym[i] = c;
        } else {
            // Handle the case where the index is out of bounds
            panic!("Index out of bounds");
        }
    }
    sym[size] = 0;
}

fn encode(sym: &[u8], size: usize, pass: &mut [usize]) {
    let mut table = b"abcdefghijklmnopqrstuvwxyz".to_vec();
    for i in 0..size {
        // Modified: Ensure the index `i` does not exceed the bounds of `pass`
        if i < sym.len() && i < pass.len() {
            let c = sym[i];
            pass[i] = move_to_front(&mut table, c);
        } else {
            // Handle the case where the index is out of bounds
            panic!("Index out of bounds");
        }
    }
}

fn check(sym: &[u8], size: usize, pass: &[usize]) -> bool {
    let mut pass2 = vec![0; size];
    let mut sym2 = vec![0; size];
    let mut val = true;

    encode(sym, size, &mut pass2);
    let mut i = 0;
    while i < size && pass[i] == pass2[i] {
        i += 1;
    }
    if i != size {
        val = false;
    }

    // Modified: Ensure that `size` does not exceed the bounds of `sym2`
    if size <= sym2.len() {
        decode(pass, size, &mut sym2);
        if sym != &sym2[..size] {
            val = false;
        }
    } else {
        // Handle the case where the size is out of bounds
        panic!("Size out of bounds");
    }

    val
}

fn main() {
    let sym = vec![
        b"broood\0".to_vec(),
        b"bananaaa\0".to_vec(),
        b"hiphophiphop\0".to_vec(),
    ];
    let mut pass = vec![0; MAX_SIZE];
    let mut i = 0;

    while i < 3 {
        // Modified: Ensure that `i` does not exceed the bounds of `sym`
        if i < sym.len() {
            let len = sym[i].len() - 1;
            encode(&sym[i], len, &mut pass);
            print!("{} : [", unsafe { str::from_utf8_unchecked(&sym[i][..len]) });
            for j in 0..len {
                print!("{} ", pass[j]);
            }
            println!("]");
            if check(&sym[i], len, &pass) {
                println!("Correct :)");
            } else {
                println!("Incorrect :(");
            }
        } else {
            // Handle the case where the index is out of bounds
            panic!("Index out of bounds");
        }
        i += 1;
    }
}