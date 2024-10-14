const MAX_SIZE: usize = 100;

fn move_to_front(s: &mut [u8], c: u8) -> usize {
    let pos = s.iter().position(|&x| x == c).unwrap();
    let mut temp = [0; MAX_SIZE];
    temp[..s.len()].copy_from_slice(s);
    s[1..=pos].copy_from_slice(&temp[..pos]);
    s[0] = c;
    pos
}

fn decode(pass: &[usize], size: usize, sym: &mut [u8]) {
    let mut table = b"abcdefghijklmnopqrstuvwxyz".to_vec();
    for i in 0..size {
        let c = table[pass[i]];
        let index = move_to_front(&mut table, c);
        if pass[i] != index {
            println!("there is an error");
        }
        sym[i] = c;
    }
    sym[size] = b'\0';
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
    let mut sym2 = vec![0; size + 1];
    encode(sym, size, &mut pass2);
    let mut i = 0;
    while i < size && pass[i] == pass2[i] {
        i += 1;
    }
    if i != size {
        return false;
    }
    decode(pass, size, &mut sym2);
    if sym != &sym2[..size] {
        return false;
    }
    true
}

fn main() {
    let sym = ["broood", "bananaaa", "hiphophiphop"];
    let mut pass = [0; MAX_SIZE];
    for i in 0..3 {
        let len = sym[i].len();
        encode(sym[i].as_bytes(), len, &mut pass);
        print!("{} : [", sym[i]);
        for j in 0..len {
            print!("{} ", pass[j]);
        }
        println!("]");
        if check(sym[i].as_bytes(), len, &pass) {
            println!("Correct :)");
        } else {
            println!("Incorrect :(");
        }
    }
}