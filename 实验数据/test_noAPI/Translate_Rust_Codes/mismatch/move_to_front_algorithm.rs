use std::ptr;
use std::str;

const MAX_SIZE: usize = 100;

fn move_to_front(s: &mut [u8], c: u8) -> usize {
    let mut p = Vec::with_capacity(s.len() + 1);
    p.extend_from_slice(s);
    p.push(0);

    // Handle the case where the element is not found by returning the length of the slice
    let q = p.iter().position(|&x| x == c).unwrap_or(s.len());
    let shift = q;

    // Ensure that `shift` does not exceed the bounds of the slice
    if shift < s.len() {
        unsafe {
            ptr::copy(s.as_ptr(), s.as_mut_ptr().offset(1), shift);
        }
    }
    s[0] = c;

    shift
}

fn decode(pass: &[usize], size: usize, sym: &mut Vec<u8>) {
    let mut table = b"abcdefghijklmnopqrstuvwxyz".to_vec();
    for i in 0..size {
        let c = table[pass[i]];
        let index = move_to_front(&mut table, c);
        if pass[i] != index {
            println!("there is an error");
        }
        sym[i] = c;
    }
    // Ensure that `sym` has enough capacity to store `size + 1` elements
    if sym.len() <= size {
        sym.resize(size + 1, 0);
    }
    sym[size] = 0;
}

fn encode(sym: &[u8], size: usize, pass: &mut [usize]) {
    let mut table = b"abcdefghijklmnopqrstuvwxyz".to_vec();
    for i in 0..size {
        let c = sym[i];
        pass[i] = move_to_front(&mut table, c);
    }
}

fn check(sym: &[u8], size: usize, pass: &[usize]) -> bool {
    let mut pass2 = vec![0; size];
    // Ensure that `sym2` is initialized with the correct capacity
    let mut sym2 = vec![0; size];
    if sym2.len() < size {
        sym2.resize(size, 0);
    }
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
    if sym != sym2 {
        val = false;
    }

    val
}

fn main() {
    let sym = vec![
        Vec::from(b"broood"),
        Vec::from(b"bananaaa"),
        Vec::from(b"hiphophiphop"),
    ];
    let mut pass = vec![0; MAX_SIZE];
    let mut i = 0;
    while i < 3 {
        // Ensure that `i` is within the bounds of the `sym` vector
        if i < sym.len() {
            let len = sym[i].len();
            encode(&sym[i], len, &mut pass);
            print!("{} : [", str::from_utf8(&sym[i]).unwrap());
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
        i += 1;
    }
}