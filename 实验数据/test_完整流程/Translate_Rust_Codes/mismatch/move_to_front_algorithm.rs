use std::str::Chars;

const MAX_SIZE: usize = 100;

// Modified: Changed the function signature to accept a `&mut String` instead of `&mut str`
fn move_to_front(s: &mut String, c: char) -> Option<usize> {
    // Handle the case where `find` returns `None`
    if let Some(q) = s.find(c) {
        let shift = q;
        let mut chars: Vec<char> = s.chars().take(shift).collect();
        chars.insert(0, c); // Ensure the character is correctly inserted at the front of the table
        *s = chars.into_iter().collect(); // Ensure the string is correctly assigned
        Some(shift)
    } else {
        // Ensure the character is added to the table if it is not found
        s.insert(0, c);
        Some(0)
    }
}

fn decode(pass: &[usize], size: usize, sym: &mut [char]) {
    let mut table = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in 0..size {
        // Ensure the index is within the bounds of the table
        if pass[i] < table.len() {
            if let Some(c) = table.chars().nth(pass[i]) {
                if let Some(index) = move_to_front(&mut table, c) {
                    if pass[i] != index {
                        println!("there is an error");
                    }
                    sym[i] = c;
                } else {
                    println!("Character not found in table");
                }
            } else {
                println!("Character not found in table");
            }
        } else {
            println!("Index out of bounds");
        }
    }
    // Ensure `sym` has enough space to accommodate the null terminator
    if size < sym.len() {
        sym[size] = '\0';
    } else {
        println!("Buffer overflow prevented");
    }
}

fn encode(sym: &str, size: usize, pass: &mut [usize]) {
    let mut table = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in 0..size {
        // Ensure the character is correctly encoded
        if let Some(c) = sym.chars().nth(i) {
            if let Some(index) = move_to_front(&mut table, c) {
                pass[i] = index;
            } else {
                println!("Character not found in table");
            }
        } else {
            println!("Index out of bounds");
        }
    }
}

fn check(sym: &str, size: usize, pass: &[usize]) -> bool {
    let mut pass2 = vec![0; size];
    let mut sym2 = vec!['\0'; size + 1]; // Ensure `sym2` has enough space for null terminator
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
    // Ensure `sym2` is correctly null-terminated and matches the expected string
    if sym != sym2[..size].iter().collect::<String>().as_str() {
        val = false;
    }

    val
}

fn main() {
    let sym = ["broood", "bananaaa", "hiphophiphop"];
    let mut pass = [0; MAX_SIZE];
    for i in 0..3 {
        let len = sym[i].len();
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