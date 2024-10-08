fn string_repeat(n: i32, s: &str) -> String {
    let slen = s.len();
    let mut dest = Vec::with_capacity((n as usize) * slen);

    unsafe {
        let mut p = dest.as_mut_ptr();
        for _ in 0..n {
            std::ptr::copy_nonoverlapping(s.as_ptr(), p, slen);
            p = p.add(slen);
        }
        dest.set_len((n as usize) * slen);
    }

    unsafe { String::from_utf8_unchecked(dest) }
}

fn main() {
    let result = string_repeat(5, "ha");
    println!("{}", result);
}