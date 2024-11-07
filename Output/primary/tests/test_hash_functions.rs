use primary::hash_pointer::*;
use primary::hash_int::*;
use primary::hash_string::*;


#[test]
pub fn test_pointer_hash() {
    const NUM_TEST_VALUES: usize = 100; // Adjust the value as needed
    let mut array = [0; NUM_TEST_VALUES];
    let mut i: usize;
    let mut j: usize;

    /* Initialise the array to all zeros */

    for i in 0..NUM_TEST_VALUES {
        array[i] = 0;
    }

    /* Check hashes are never the same */

    for i in 0..NUM_TEST_VALUES {
        for j in (i + 1)..NUM_TEST_VALUES {
            // Modified: Correct casting to ensure valid casting from &i32 to *const ()
            assert!(pointer_hash(&array[i] as *const i32 as *const ()) != pointer_hash(&array[j] as *const i32 as *const ()));
        }
    }
}
// Renamed type `byte` to `Byte` to follow Rust's naming convention for types
pub type Byte = u8;

// Removed parameters from the `process_data` function as it is not intended to be a test
#[test]
fn process_data() {
    let a = 5;
    let b = 10;
    let str = "hello";
    
    let sum = add_in_rust(a, b);
    let product = multiply_in_rust(a, b);
    let reversed = reverse_string_in_rust(str);
    
    // Use `println!` macro to ensure output is printed on a new line
    println!("Sum: {}", sum);
    println!("Product: {}", product);
    println!("Reversed string: {}", reversed);

    // Removed unnecessary `drop` call
}

#[test]
fn test_int_hash() {
    let mut array = [0; NUM_TEST_VALUES];
    // Removed unnecessary declarations of `i` and `j`

    /* Initialise all entries in the array */

    for i in 0..NUM_TEST_VALUES {
        array[i] = i;
    }

    /* Check hashes are never the same */

    for i in 0..NUM_TEST_VALUES {
        for j in (i + 1)..NUM_TEST_VALUES {
            assert!(int_hash(&mut array[i]) != int_hash(&mut array[j]));
        }
    }

    /* Hashes of two variables containing the same value are the same */

    let i = 5000;
    let j = 5000;

    assert!(int_hash(&mut i) == int_hash(&mut j));
}
#[test]
pub fn process_data(a: i32, b: i32, str: &str) {
    let sum = add_in_rust(a, b);
    let product = multiply_in_rust(a, b);
    let reversed = reverse_string_in_rust(str);
    
    print!("Sum: {}\n", sum);
    print!("Product: {}\n", product);
    print!("Reversed string: {}\n", reversed);

    // Removed: Unused variable `reversed` is dropped immediately after printing
    // drop(reversed);
}
#[test]
pub fn test_string_hash() {
    let test1 = "this is a test";
    let test2 = "this is a tesu";
    let test3 = "this is a test ";
    let test4 = "this is a test";
    let test5 = "This is a test";

    /* Contents affect the hash */

    assert!(string_hash(test1.as_bytes()) != string_hash(test2.as_bytes()));

    /* Length affects the hash */

    assert!(string_hash(test1.as_bytes()) != string_hash(test3.as_bytes()));

    /* Case sensitive */

    assert!(string_hash(test1.as_bytes()) != string_hash(test5.as_bytes()));

    /* The same strings give the same hash */

    assert!(string_hash(test1.as_bytes()) == string_hash(test4.as_bytes()));
}