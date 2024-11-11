
use primary::hash_string::*;

pub fn string_nocase_hash(string: &str) -> u32 {
    let mut result: u32 = 5381;
    let mut p = string.chars();

    while let Some(c) = p.next() {
        result = (result << 5).wrapping_add(result).wrapping_add(c.to_ascii_lowercase() as u32);
    }

    result
}
#[test]
pub fn test_string_nocase_hash() {
    let test1 = "this is a test";
    let test2 = "this is a tesu";
    let test3 = "this is a test ";
    let test4 = "this is a test";
    let test5 = "This is a test";

    /* Contents affect the hash */

    assert!(string_nocase_hash(test1) != string_nocase_hash(test2));

    /* Length affects the hash */

    assert!(string_nocase_hash(test1) != string_nocase_hash(test3));

    /* Case insensitive */

    assert!(string_nocase_hash(test1) == string_nocase_hash(test5));

    /* The same strings give the same hash */

    assert!(string_nocase_hash(test1) == string_nocase_hash(test4));
}