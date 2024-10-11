use std::cmp::Ordering;

const MAX_SIZE: usize = 100;

fn move_to_front(s: &mut String, c: char) -> usize {
    let mut p = s.clone();
    let q = p.find(c).unwrap();
    let shift = q;
    let mut new_str = String::new();
    new_str.push(c);
    new_str.push_str(&p.chars().take(shift).collect::<String>());
    new_str.push_str(&p.chars().skip(shift + 1).collect::<String>());
    *s = new_str;
    shift
}

fn decode(pass: &[usize], size: usize, sym: &mut [char]) {
    let mut table = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in 0..size {
        let c = table.chars().nth(pass[i]).unwrap();
        let index = move_to_front(&mut table, c);
        if pass[i] != index {
            println!("there is an error");
        }
        sym[i] = c;
    }
    sym[size] = '\0';
}

fn encode(sym: &str, size: usize, pass: &mut [usize]) {
    let mut table = "abcdefghijklmnopqrstuvwxyz".to_string();
    for i in 0..size {
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