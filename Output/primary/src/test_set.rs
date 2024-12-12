use crate::set::*;
use crate::compare_int::*;
use crate::hash_int::*;
use crate::compare_pointer::*;
use crate::hash_pointer::*;
use crate::compare_string::*;
use crate::hash_string::*;
use crate::translation_utils::*;static mut allocated_values: i32 = 0;
pub fn generate_set() -> Owned<Set<CStr>> {
    let mut set: Owned<Set<CStr>>;
    let mut buf: CStr;
    let mut i: u32;
    let mut value: CStr;
    set = set_new(func!(string_hash), func!(string_equal));
    c_for!(i = 0; i < 10000; i += 1; {
        c_sprintf!(buf, "{}", i);
        value = c_strdup!(buf);
        set_insert(set.unowned(), value);
        assert!(set_num_entries(set.unowned()) == i + 1);
    });
    set_register_free_function(set.unowned(), c_free!());
    return set;
}
#[test]
fn test_set_new_free() {
    let mut set: Owned<Set<Ptr<i32>>>;
    let mut i: i32;
    let mut value: Ptr<i32>;
    set = set_new(func!(int_hash), func!(int_equal));
    set_register_free_function(set.unowned(), c_free!());
    assert!(set != null!());
    c_for!(i = 0; i < 10000; i += 1; {
        value = c_malloc!(c_sizeof!(i32));
        *value = i;
        set_insert(set.unowned(), value);
    });
    set_free(set.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_set_insert() {
    let mut set: Owned<Set<Ptr<i32>>>;
    let mut numbers1 = arr![1, 2, 3, 4, 5, 6];
    let mut numbers2 = arr![5, 6, 7, 8, 9, 10];
    let mut i: i32;
    set = set_new(func!(int_hash), func!(int_equal));
    c_for!(i = 0; i < 6; i += 1; {
        set_insert(set.unowned(), c_ref!(numbers1[i as usize]));
    });
    c_for!(i = 0; i < 6; i += 1; {
        set_insert(set.unowned(), c_ref!(numbers2[i as usize]));
    });
    assert!(set_num_entries(set.unowned()) == 10);
    set_free(set.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_set_query() {
    let mut set: Owned<Set<CStr>>;
    let mut buf: CStr;
    let mut i: i32;
    set = generate_set();
    c_for!(i = 0; i < 10000; i += 1; {
        c_sprintf!(buf, "{}", i);
        assert!(set_query(set.unowned(), buf) != 0);
    });
    assert!(set_query(set.unowned(), cstr!("-1")) == 0);
    assert!(set_query(set.unowned(), cstr!("100001")) == 0);
    set_free(set.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_set_remove() {
    let mut set: Owned<Set<CStr>>;
    let mut buf: CStr;
    let mut i: i32;
    let mut num_entries: u32;
    set = generate_set();
    num_entries = set_num_entries(set.unowned());
    assert!(num_entries == 10000);
    c_for!(i = 4000; i < 6000; i += 1; {
        c_sprintf!(buf, "{}", i);
        assert!(set_query(set.unowned(), buf) != 0);
        assert!(set_remove(set.unowned(), buf) != 0);
        assert!(set_num_entries(set.unowned()) == num_entries - 1);
        assert!(set_query(set.unowned(), buf) == 0);
        num_entries -= 1;
    });
    c_for!(i = -1000; i < -500; i += 1; {
        c_sprintf!(buf, "{}", i);
        assert!(set_remove(set.unowned(), buf) == 0);
        assert!(set_num_entries(set.unowned()) == num_entries);
    });
    c_for!(i = 50000; i < 51000; i += 1; {
        c_sprintf!(buf, "{}", i);
        assert!(set_remove(set.unowned(), buf) == 0);
        assert!(set_num_entries(set.unowned()) == num_entries);
    });
    set_free(set.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_set_union() {
    let mut numbers1 = arr![1, 2, 3, 4, 5, 6, 7];
    let mut numbers2 = arr![5, 6, 7, 8, 9, 10, 11];
    let mut result = arr![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut i: i32;
    let mut set1: Owned<Set<Ptr<i32>>>;
    let mut set2: Owned<Set<Ptr<i32>>>;
    let mut result_set: Owned<Set<Ptr<i32>>>;
    let mut allocated: usize;

    set1 = set_new(func!(int_hash), func!(int_equal));
    c_for!(i = 0; i < 7; i += 1; {
        set_insert(set1.unowned(), c_ref!(numbers1[i as usize]));
    });

    set2 = set_new(func!(int_hash), func!(int_equal));
    c_for!(i = 0; i < 7; i += 1; {
        set_insert(set2.unowned(), c_ref!(numbers2[i as usize]));
    });

    result_set = set_union(set1.unowned(), set2.unowned());
    assert!(set_num_entries(result_set.unowned()) == 11);

    c_for!(i = 0; i < 11; i += 1; {
        assert!(set_query(result_set.unowned(), c_ref!(result[i as usize])) != 0);
    });

    set_free(result_set.unowned());
    set_free(set1.unowned());
    set_free(set2.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_set_intersection() {
    let mut numbers1 = arr![1, 2, 3, 4, 5, 6, 7];
    let mut numbers2 = arr![5, 6, 7, 8, 9, 10, 11];
    let mut result = arr![5, 6, 7];
    let mut i: i32;
    let mut set1: Owned<Set<Ptr<i32>>>;
    let mut set2: Owned<Set<Ptr<i32>>>;
    let mut result_set: Owned<Set<Ptr<i32>>>;
    let mut allocated: usize;

    set1 = set_new(func!(int_hash), func!(int_equal));
    c_for!(i = 0; i < 7; i += 1; {
        set_insert(set1.unowned(), c_ref!(numbers1[i as usize]));
    });

    set2 = set_new(func!(int_hash), func!(int_equal));
    c_for!(i = 0; i < 7; i += 1; {
        set_insert(set2.unowned(), c_ref!(numbers2[i as usize]));
    });

    result_set = set_intersection(set1.unowned(), set2.unowned());
    assert!(set_num_entries(result_set.unowned()) == 3);

    c_for!(i = 0; i < 3; i += 1; {
        assert!(set_query(result_set.unowned(), c_ref!(result[i as usize])) != 0);
    });

    set_free(set1.unowned());
    set_free(set2.unowned());
    set_free(result_set.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_set_to_array() {
    let mut set: Owned<Set<Ptr<i32>>>;
    let mut values = arr![1; 100];
    let mut array: Vector<Ptr<i32>>;
    let mut i: i32;

    set = set_new(func!(pointer_hash), func!(pointer_equal));

    c_for!(i = 0; i < 100; i += 1; {
        values[i as usize] = 1;
        set_insert(set.unowned(), c_ref!(values[i as usize]));
    });

    array = set_to_array(set.unowned());

    c_for!(i = 0; i < 100; i += 1; {
        assert!(*array[i as usize] == 1);
        *array[i as usize] = 0;
    });

    c_free!(array);
    set_free(set.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_set_iterating() {
    let mut set: Owned<Set<Ptr<i32>>>;
    let mut iterator: SetIterator<Ptr<i32>> = Default::default();
    let mut count: i32;
    set = generate_set();
    count = 0;
    set_iterate(set.unowned(), c_ref!(iterator));
    while set_iter_has_more(c_ref!(iterator)).as_bool() {
        set_iter_next(c_ref!(iterator));
        count += 1;
    }
    assert!(set_iter_next(c_ref!(iterator)) == set_null!());
    assert!(count == 10000);
    set_free(set.unowned());
    let mut set: Owned<Set<Ptr<i32>>>;
    let mut iterator: SetIterator<Ptr<i32>> = Default::default();
    set = set_new(func!(int_hash), func!(int_equal));
    set_iterate(set.unowned(), c_ref!(iterator));
    assert!(set_iter_has_more(c_ref!(iterator)) == 0);
    set_free(set.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_set_iterating_remove() {
    let mut set: Owned<Set<CStr>>;
    let mut iterator: SetIterator<CStr> = Default::default();
    let mut count: i32;
    let mut removed: u32;
    let mut value: CStr;
    set = generate_set();
    count = 0;
    removed = 0;
    set_iterate(set.unowned(), c_ref!(iterator));
    while set_iter_has_more(c_ref!(iterator)).as_bool() {
        value = set_iter_next(c_ref!(iterator));
        if (c_atoi!(value) % 100) == 0 {
            set_remove(set.unowned(), value);
            removed += 1;
        }
        count += 1;
    }
    assert!(count == 10000);
    assert!(removed == 100);
    assert!(set_num_entries(set.unowned()) == 10000 - removed);
    set_free(set.unowned());
    test_no_memory_leak!();
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
fn test_set_free_function() {
    unsafe {
        let mut set: Owned<Set<Ptr<i32>>>;
        let mut i: i32;
        let mut value: Ptr<i32>;
        set = set_new(func!(int_hash), func!(int_equal));
        set_register_free_function(set.unowned(), func!(free_value));
        allocated_values = 0;
        c_for!(i = 0; i < 1000; i += 1; {
            value = new_value(i);
            set_insert(set.unowned(), value);
        });
        assert!(allocated_values == 1000);
        i = 500;
        set_remove(set.unowned(), c_ref!(i));
        assert!(allocated_values == 999);
        set_free(set.unowned());
        assert!(allocated_values == 0);
        test_no_memory_leak!();
    }
}
