fn string_repeat(n: usize, s: &str) -> String {
    let slen = s.len();
    let mut dest = String::with_capacity(n * slen);

    for _ in 0..n {
        dest.push_str(s);
    }

    dest
}

fn main() {
    let result = string_repeat(5, "ha");
    println!("{}", result);
}