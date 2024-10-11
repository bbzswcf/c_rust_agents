use std::cmp::Ordering;

const MAX_SIZE: usize = 100;

fn move_to_front(s: &mut String, c: char) -> usize {
    let p = s.clone();
    // Handle the case where the character is not found in the string
    let q = match p.find(c) {
        Some(index) => index,
        None => {
            println!("Character '{}' not found in the string", c);
            return 0; // or handle the error in a way that makes sense for your program
        }
    };
    let shift = q;
    let mut new_str = String::new();
    new_str.push(c);
    new_str.push_str(&p.chars().take(shift).collect::<String>()); // Correctly rearrange the rest of the string
    new_str.push_str(&p.chars().skip(shift + 1).collect::<String>());
    *s = new_str;
    shift
}

fn decode(pass: &[usize], size: usize, sym: &mut [char]) {
    let mut table = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in 0..size {
        // Handle the case where the index is out of bounds
        if pass[i] >= table.len() {
            println!("Index out of bounds: {}", pass[i]);
            return; // or handle the error in a way that makes sense for your program
        }
        let c = table.chars().nth(pass[i]).unwrap();
        let index = move_to_front(&mut table, c);
        if pass[i] != index {
            println!("there is an error");
        }
        sym[i] = c;
    }
    // Ensure the index is within the bounds of the vector
    if size < sym.len() {
        sym[size] = '\0';
    } else {
        println!("Index out of bounds: {}", size);
    }
}

fn encode(sym: &str, size: usize, pass: &mut [usize]) {
    let mut table = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in 0..size {
        // Handle the case where the index is out of bounds
        if i >= sym.len() {
            println!("Index out of bounds: {}", i);
            return; // or handle the error in a way that makes sense for your program
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
    for i in 0..sym.len() {
        let len = sym[i].len();
        if len > pass.len() {
            println!("Pass array is too small for the encoded values");
            continue; // or handle the error in a way that makes sense for your program
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