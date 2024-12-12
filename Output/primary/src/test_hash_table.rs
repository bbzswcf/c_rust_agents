use crate::hash_table::*;
use crate::hash_int::*;
use crate::compare_int::*;
use crate::hash_string::*;
use crate::compare_string::*;
use crate::translation_utils::*;macro_rules! NUM_TEST_VALUES { () => { 10000 } }

const value1: i32 = 1;
const value2: i32 = 2;
const value3: i32 = 3;
const value4: i32 = 4;

static mut allocated_keys: i32 = 0;
static mut allocated_values: i32 = 0;
pub fn generate_hash_table() -> Owned<HashTable<CStr>> {
    let mut hash_table: Owned<HashTable<CStr>>;
    let mut buf: CStr;
    let mut value: CStr;
    let mut i: i32;
    hash_table = hash_table_new(func!(string_hash), func!(string_equal));
    c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
        c_sprintf!(buf, "{}", i);
        value = c_strdup!(buf);
        hash_table_insert(hash_table.unowned(), value, value);
    });
    hash_table_register_free_functions(hash_table.unowned(), null!(), c_free!());
    return hash_table;
}
#[test]
fn test_hash_table_new_free() {
    let mut hash_table: Owned<HashTable<Ptr<i32>>>;
    hash_table = hash_table_new(func!(int_hash), func!(int_equal));
    assert!(hash_table != null!());
    hash_table_insert(hash_table.unowned(), c_ref!(value1), c_ref!(value1));
    hash_table_insert(hash_table.unowned(), c_ref!(value2), c_ref!(value2));
    hash_table_insert(hash_table.unowned(), c_ref!(value3), c_ref!(value3));
    hash_table_insert(hash_table.unowned(), c_ref!(value4), c_ref!(value4));
    hash_table_free(hash_table.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_hash_table_insert_lookup() {
    let mut hash_table: Owned<HashTable<CStr>>;
    let mut buf: CStr;
    let mut value: CStr;
    let mut i: i32;

    hash_table = generate_hash_table();
    assert!(hash_table_num_entries(hash_table.unowned()) == NUM_TEST_VALUES!());

    c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
        c_sprintf!(buf, "{}", i);
        value = hash_table_lookup(hash_table.unowned(), buf);
        assert!(c_strcmp!(value, buf) == 0);
    });

    c_sprintf!(buf, "{}", -1);
    assert!(hash_table_lookup(hash_table.unowned(), buf) == null!());

    c_sprintf!(buf, "{}", NUM_TEST_VALUES!());
    assert!(hash_table_lookup(hash_table.unowned(), buf) == null!());

    c_sprintf!(buf, "{}", 12345);
    hash_table_insert(hash_table.unowned(), buf, c_strdup!(cstr!("hello world")));
    value = hash_table_lookup(hash_table.unowned(), buf);
    assert!(c_strcmp!(value, cstr!("hello world")) == 0);

    hash_table_free(hash_table.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_hash_table_remove() {
    let mut hash_table: Owned<HashTable<CStr>>;
    let mut buf: CStr;
    hash_table = generate_hash_table();
    assert!(hash_table_num_entries(hash_table.unowned()) == NUM_TEST_VALUES!());
    c_sprintf!(buf, "{}", 5000);
    assert!(hash_table_lookup(hash_table.unowned(), buf) != null!());
    hash_table_remove(hash_table.unowned(), buf);
    assert!(hash_table_num_entries(hash_table.unowned()) == NUM_TEST_VALUES!() - 1);
    assert!(hash_table_lookup(hash_table.unowned(), buf) == null!());
    c_sprintf!(buf, "{}", -1);
    hash_table_remove(hash_table.unowned(), buf);
    assert!(hash_table_num_entries(hash_table.unowned()) == NUM_TEST_VALUES!() - 1);
    hash_table_free(hash_table.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_hash_table_iterating() {
    let mut hash_table: Owned<HashTable<CStr>> = generate_hash_table();
    let mut iterator: HashTableIterator<CStr> = Default::default();
    let mut count: i32;

    count = 0;
    hash_table_iterate(hash_table.unowned(), c_ref!(iterator));
    while hash_table_iter_has_more(c_ref!(iterator)).as_bool() {
        hash_table_iter_next(c_ref!(iterator));
        count += 1;
    }
    assert!(count == NUM_TEST_VALUES!());
    let pair = hash_table_iter_next(c_ref!(iterator));
    assert!(pair.value.is_null());
    hash_table_free(hash_table.unowned());

    let mut hash_table: Owned<HashTable<Ptr<i32>>> = hash_table_new(func!(int_hash), func!(int_equal));
    hash_table_iterate(hash_table.unowned(), c_ref!(iterator));
    assert!(hash_table_iter_has_more(c_ref!(iterator)) == 0);
    hash_table_free(hash_table.unowned());

    test_no_memory_leak!();
}
#[test]
fn test_hash_table_iterating_remove() {
    let mut hash_table: Owned<HashTable<CStr>>;
    let mut iterator: HashTableIterator<CStr> = Default::default();
    let mut buf: CStr;
    let mut val: CStr;
    let mut pair: HashTablePair<CStr>;
    let mut count: i32;
    let mut removed: u32;
    let mut i: i32;
    hash_table = generate_hash_table();
    count = 0;
    removed = 0;
    hash_table_iterate(hash_table.unowned(), c_ref!(iterator));
    while hash_table_iter_has_more(c_ref!(iterator)).as_bool() {
        pair = hash_table_iter_next(c_ref!(iterator));
        val = pair.value;
        if (c_atoi!(val) % 100) == 0 {
            hash_table_remove(hash_table.unowned(), val);
            removed += 1;
        }
        count += 1;
    }
    assert!(removed == 100);
    assert!(count == NUM_TEST_VALUES!());
    assert!(hash_table_num_entries(hash_table.unowned())
        == NUM_TEST_VALUES!() - removed);
    c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
        c_sprintf!(buf, "{}", i);
        if i % 100 == 0 {
            assert!(hash_table_lookup(hash_table.unowned(), buf) == null!());
        } else {
            assert!(hash_table_lookup(hash_table.unowned(), buf) != null!());
        }
    });
    hash_table_free(hash_table.unowned());
    test_no_memory_leak!();
}
fn new_key(mut value: i32) -> Ptr<i32> {
    unsafe {
        let mut result: Ptr<i32>;
        result = c_malloc!(c_sizeof!(i32));
        *result = value;
        allocated_keys += 1;
        result
    }
}
fn free_key(mut key: Ptr<i32>) {
	unsafe {
		c_free!(key);
		allocated_keys -= 1;
	}
}
fn new_value(mut value: i32) -> Ptr<i32> {
    unsafe {
        let mut result: Ptr<i32>;
        result = c_malloc!(c_sizeof!(i32));
        *result = value;
        allocated_values += 1;
        result
    }
}
fn free_value(mut value: Ptr<i32>) {
	unsafe {
		c_free!(value);
		allocated_values -= 1;
	}
}
#[test]
pub fn test_hash_table_free_functions() {
    unsafe {
        let mut hash_table: Owned<HashTable<Ptr<i32>>>;
        let mut key: Ptr<i32>;
        let mut value: Ptr<i32>;
        let mut i: i32;
        hash_table = hash_table_new(func!(int_hash), func!(int_equal));
        hash_table_register_free_functions(hash_table.unowned(), func!(free_key), func!(free_value));
        allocated_values = 0;
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            key = new_key(i);
            value = new_value(99);
            hash_table_insert(hash_table.unowned(), key, value);
        });
        assert!(allocated_keys == NUM_TEST_VALUES!());
        assert!(allocated_values == NUM_TEST_VALUES!());
        i = NUM_TEST_VALUES!() / 2;
        hash_table_remove(hash_table.unowned(), c_ref!(i));
        assert!(allocated_keys == NUM_TEST_VALUES!() - 1);
        assert!(allocated_values == NUM_TEST_VALUES!() - 1);
        key = new_key(NUM_TEST_VALUES!() / 3);
        value = new_value(999);
        assert!(allocated_keys == NUM_TEST_VALUES!());
        assert!(allocated_values == NUM_TEST_VALUES!());
        hash_table_insert(hash_table.unowned(), key, value);
        assert!(allocated_keys == NUM_TEST_VALUES!() - 1);
        assert!(allocated_values == NUM_TEST_VALUES!() - 1);
        hash_table_free(hash_table.unowned());
        assert!(allocated_keys == 0);
        assert!(allocated_values == 0);
        test_no_memory_leak!();
    }
}
#[test]
fn test_hash_iterator_key_pair() {
    let mut hash_table: Owned<HashTable<Ptr<i32>>>;
    let mut iterator: HashTableIterator<Ptr<i32>> = Default::default();
    let mut pair: HashTablePair<Ptr<i32>>;

    hash_table = hash_table_new(func!(int_hash), func!(int_equal));

    hash_table_insert(hash_table.unowned(), c_ref!(value1), c_ref!(value1));
    hash_table_insert(hash_table.unowned(), c_ref!(value2), c_ref!(value2));

    hash_table_iterate(hash_table.unowned(), c_ref!(iterator));

    while hash_table_iter_has_more(c_ref!(iterator)).as_bool() {
        pair = hash_table_iter_next(c_ref!(iterator));

        let key = unsafe { *pair.key };
        let val = unsafe { *pair.value };

        assert!(key == val);
    }

    hash_table_free(hash_table.unowned());
}
