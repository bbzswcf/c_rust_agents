

pub fn string_nocase_hash(string: &str) -> u32 {
    let mut result: u32 = 5381;
    let mut p = string.chars();

    while let Some(c) = p.next() {
        result = (result << 5).wrapping_add(result).wrapping_add(c.to_ascii_lowercase() as u32);
    }

    result
}