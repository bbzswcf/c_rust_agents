use std::cmp::Ordering;

const MAX_SIZE: usize = 100;

fn move_to_front(s: &mut String, c: char) -> usize {
    // Correctly move the character to the front and shift the rest of the characters
    if let Some(index) = s.find(c) {
        s.remove(index);
        s.insert(0, c);
        index
    } else {
        0
    }
}

fn decode(pass: &[usize], size: usize, sym: &mut [char]) {
    let mut table = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in 0..size {
        // Ensure the index is within bounds before accessing the character
        if pass[i] < table.len() {
            if let Some(c) = table.chars().nth(pass[i]) {
                let index = move_to_front(&mut table, c);
                if pass[i] != index {
                    println!("there is an error");
                }
                sym[i] = c;
            } else {
                // Handle the case where the character is not found
                println!("Character not found at index {}", pass[i]);
            }
        } else {
            // Handle the case where the index is out of bounds
            println!("Index out of bounds: {}", pass[i]);
        }
    }
    // Ensure sym has enough space for the null terminator
    if size < sym.len() {
        sym[size] = '\0';
    } else {
        println!("sym array is too small to add null terminator");
    }
}

fn encode(sym: &str, size: usize, pass: &mut [usize]) {
    let mut table = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in 0..size {
        // Ensure the index is within bounds before accessing the character
        if let Some(c) = sym.chars().nth(i) {
            pass[i] = move_to_front(&mut table, c);
        } else {
            // Handle the case where the character is not found
            println!("Character not found at index {}", i);
        }
    }
}

fn check(sym: &str, size: usize, pass: &[usize]) -> bool {
    let mut pass2 = vec![0; size];
    // Ensure sym2 has enough space for the null terminator
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
    if sym.cmp(&sym2[..size].iter().collect::<String>()) != Ordering::Equal {
        val = false;
    }

    val
}

fn main() {
    let sym = vec!["broood", "bananaaa", "hiphophiphop"];
    let mut pass = vec![0; MAX_SIZE];
    for i in 0..3 {
        // Ensure sym[i] is a valid string slice before calling len()
        if let Some(s) = sym.get(i) {
            let len = s.len();
            encode(s, len, &mut pass);
            print!("{} : [", s);
            for j in 0..len {
                print!("{} ", pass[j]);
            }
            println!("]");
            if check(s, len, &pass) {
                println!("Correct :)");
            } else {
                println!("Incorrect :(");
            }
        } else {
            println!("Invalid string slice at index {}", i);
        }
    }
}