fn lcs<'a>(sa: &'a str, sb: &'a str, beg: &mut &'a str, end: &mut &'a str) {
    // Modified: Ensured all references in the function signature share the same lifetime 'a
    let sa_bytes = sa.as_bytes();
    let sb_bytes = sb.as_bytes();

    *beg = "";
    *end = "";

    for (apos, &a_char) in sa_bytes.iter().enumerate() {
        for (bpos, &b_char) in sb_bytes.iter().enumerate() {
            if a_char == b_char {
                let mut len = 1; // Modified: Initialize len directly within the loop
                // Modified: Ensured the indices used in the array access are within bounds
                while apos + (len as usize) < sa_bytes.len() && bpos + (len as usize) < sb_bytes.len() && sa_bytes[apos + len as usize] == sb_bytes[bpos + len as usize] {
                    len += 1;
                }

                // Modified: Compared the lengths directly using the len() method on the slices
                if len > end.len() - beg.len() {
                    *beg = &sa[apos..apos + len as usize]; // Modified: Corrected the slice assignment
                    *end = &sb[bpos + len as usize..]; // Modified: Corrected the slice assignment
                }
            }
        }
    }
}

fn main() {
    let s1 = "thisisatest";
    let s2 = "testing123testing";
    let mut beg: &str = ""; // Modified: Correctly initialized as a mutable reference to an empty string
    let mut end: &str = ""; // Modified: Correctly initialized as a mutable reference to an empty string

    lcs(s1, s2, &mut beg, &mut end);

    // Modified: Ensure the loop iterates over the updated value of beg after the lcs function call
    for it in beg.chars() {
        print!("{}", it);
    }
    println!();
}