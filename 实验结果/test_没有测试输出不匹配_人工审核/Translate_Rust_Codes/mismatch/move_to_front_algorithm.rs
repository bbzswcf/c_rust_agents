// Removed unstable feature attribute
// #![feature(repr128)]

use std::cmp::Ordering;

const MAX_SIZE: usize = 100;

fn move_to_front(str: &mut String, c: char) -> Option<usize> {
    // Avoid unnecessary cloning by using references
    if let Some(q) = str.find(c) {
        let shift = q;
        // Modify the existing string in place
        str.push(c);
        str.push_str(&str.chars().take(shift).collect::<String>());
        Some(shift)
    } else {
        None
    }
}

fn decode(pass: &[usize], size: usize, sym: &mut [char]) {
    let mut table = String::from("abcdefghijklmnopqrstuvwxyz");
    for i in 0..size {
        // Ensure that `pass[i]` is within the bounds of the `table` string
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
                println!("Index out of bounds");
            }
        } else {
            println!("Index out of bounds");
        }
    }
    // Ensure that the index `size - 1` is within the bounds of the `sym` array
    if size > 0 {
        sym[size - 1] = '\0';
    }
}

fn encode(sym: &str, size: usize, pass: &mut [usize]) {
    let mut table = String::from("abcdefghijklmnopqrstuvwxyz");
    for i in 0..size {
        // Ensure that `i` is within the bounds of the `sym` string
        if i < sym.len() {
            if let Some(c) = sym.chars().nth(i) {
                if let Some(index) = move_to_front(&mut table, c) {
                    pass[i] = index;
                } else {
                    println!("Character not found in table");
                }
            } else {
                println!("Index out of bounds");
            }
        } else {
            println!("Index out of bounds");
        }
    }
}

fn check(sym: &str, size: usize, pass: &[usize]) -> bool {
    let mut pass2 = vec![0; size];
    let mut sym2 = vec!['\0'; size];
    let mut val = true;

    encode(sym, size, &mut pass2);
    let mut i = 0;
    // Ensure that `i` is within the bounds of the `pass` and `pass2` arrays
    while i < size && i < pass.len() && i < pass2.len() && pass[i] == pass2[i] {
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
    let sym = vec![
        String::from("broood"),
        String::from("bananaaa"),
        String::from("hiphophiphop"),
    ];
    let mut pass = vec![0; MAX_SIZE];
    for i in 0..3 {
        // Ensure that `i` is within the bounds of the `sym` vector
        if i < sym.len() {
            let len = sym[i].len();
            // Ensure that `len` is within the bounds of `pass`
            if len <= pass.len() {
                encode(&sym[i], len, &mut pass);
                print!("{} : [", sym[i]);
                for j in 0..len {
                    print!("{} ", pass[j]);
                }
                println!("]");
                // Ensure that `i` is within the bounds of the `sym` vector
                if i < sym.len() {
                    if check(&sym[i], len, &pass) {
                        println!("Correct :)");
                    } else {
                        println!("Incorrect :(");
                    }
                } else {
                    println!("Index out of bounds");
                }
            } else {
                println!("Length out of bounds");
            }
        } else {
            println!("Index out of bounds");
        }
    }
}