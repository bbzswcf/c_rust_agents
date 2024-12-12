use crate::list::*;
use crate::compare_int::*;
use crate::translation_utils::*;static mut variable1: i32 = 50;
static mut variable2: i32 = 0;
static mut variable3: i32 = 0;
static mut variable4: i32 = 0;
fn generate_list() -> Manual<ListEntry<Ptr<i32>>> {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>> = null!();
        assert!(list_append(c_ref!(list), c_ref!(variable1)) != null!());
        assert!(list_append(c_ref!(list), c_ref!(variable2)) != null!());
        assert!(list_append(c_ref!(list), c_ref!(variable3)) != null!());
        assert!(list_append(c_ref!(list), c_ref!(variable4)) != null!());
        return list;
    }
}
fn check_list_integrity<T: GenericValue>(mut list: Manual<ListEntry<T>>) {
    let mut prev: Manual<ListEntry<T>> = null!();
    let mut rover: Manual<ListEntry<T>> = list;
    while rover != null!() {
        assert!(list_prev(rover) == prev);
        prev = rover;
        rover = list_next(rover);
    }
}
#[test]
fn test_list_append() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>> = null!();
        assert!(list_append(c_ref!(list), c_ref!(variable1)) != null!());
        check_list_integrity(list);
        assert!(list_append(c_ref!(list), c_ref!(variable2)) != null!());
        check_list_integrity(list);
        assert!(list_append(c_ref!(list), c_ref!(variable3)) != null!());
        check_list_integrity(list);
        assert!(list_append(c_ref!(list), c_ref!(variable4)) != null!());
        check_list_integrity(list);
        assert!(list_length(list) == 4);
        assert!(list_nth_data(list, 0) == c_ref!(variable1));
        assert!(list_nth_data(list, 1) == c_ref!(variable2));
        assert!(list_nth_data(list, 2) == c_ref!(variable3));
        assert!(list_nth_data(list, 3) == c_ref!(variable4));
        list_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_prepend() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>> = null!();
        assert!(list_prepend(c_ref!(list), c_ref!(variable1)) != null!());
        check_list_integrity(list);
        assert!(list_prepend(c_ref!(list), c_ref!(variable2)) != null!());
        check_list_integrity(list);
        assert!(list_prepend(c_ref!(list), c_ref!(variable3)) != null!());
        check_list_integrity(list);
        assert!(list_prepend(c_ref!(list), c_ref!(variable4)) != null!());
        check_list_integrity(list);
        assert!(list_nth_data(list, 0) == c_ref!(variable4));
        assert!(list_nth_data(list, 1) == c_ref!(variable3));
        assert!(list_nth_data(list, 2) == c_ref!(variable2));
        assert!(list_nth_data(list, 3) == c_ref!(variable1));
        list_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_free() {
    let mut list: Manual<ListEntry<Ptr<i32>>>;
    list = generate_list();
    list_free(list);
    list_free::<Ptr<i32>>(null!());
    test_no_memory_leak!();
}
#[test]
fn test_list_next() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>>;
        let mut rover: Manual<ListEntry<Ptr<i32>>>;
        list = generate_list();
        rover = list;
        assert!(list_data(rover) == c_ref!(variable1));
        rover = list_next(rover);
        assert!(list_data(rover) == c_ref!(variable2));
        rover = list_next(rover);
        assert!(list_data(rover) == c_ref!(variable3));
        rover = list_next(rover);
        assert!(list_data(rover) == c_ref!(variable4));
        rover = list_next(rover);
        assert!(rover == null!());
        list_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_nth_entry() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>>;
        let mut entry: Manual<ListEntry<Ptr<i32>>>;
        list = generate_list();
        entry = list_nth_entry(list, 0);
        assert!(list_data(entry) == c_ref!(variable1));
        entry = list_nth_entry(list, 1);
        assert!(list_data(entry) == c_ref!(variable2));
        entry = list_nth_entry(list, 2);
        assert!(list_data(entry) == c_ref!(variable3));
        entry = list_nth_entry(list, 3);
        assert!(list_data(entry) == c_ref!(variable4));
        entry = list_nth_entry(list, 4);
        assert!(entry == null!());
        entry = list_nth_entry(list, 400);
        assert!(entry == null!());
        list_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_nth_data() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>>;
        list = generate_list();
        assert!(list_nth_data(list, 0) == c_ref!(variable1));
        assert!(list_nth_data(list, 1) == c_ref!(variable2));
        assert!(list_nth_data(list, 2) == c_ref!(variable3));
        assert!(list_nth_data(list, 3) == c_ref!(variable4));
        assert!(list_nth_data(list, 4) == null!());
        assert!(list_nth_data(list, 400) == null!());
        list_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_length() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>>;
        list = generate_list();
        assert!(list_length(list) == 4);
        assert!(list_prepend(c_ref!(list), c_ref!(variable1)) != null!());
        assert!(list_length(list) == 5);
        list_free(list);
        assert!(list_length::<Ptr<i32>>(null!()) == 0);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_remove_entry() {
    unsafe {
        let mut empty_list: Manual<ListEntry<Ptr<i32>>> = null!();
        let mut list: Manual<ListEntry<Ptr<i32>>>;
        let mut entry: Manual<ListEntry<Ptr<i32>>>;
        list = generate_list();
        entry = list_nth_entry(list, 2);
        assert!(list_remove_entry(c_ref!(list), entry) != 0);
        assert!(list_length(list) == 3);
        check_list_integrity(list);
        entry = list_nth_entry(list, 0);
        assert!(list_remove_entry(c_ref!(list), entry) != 0);
        assert!(list_length(list) == 2);
        check_list_integrity(list);
        assert!(list_remove_entry(c_ref!(list), null!()) == 0);
        assert!(list_remove_entry(c_ref!(empty_list), null!()) == 0);
        list_free(list);
        list = null!();
        assert!(list_append(c_ref!(list), c_ref!(variable1)) != null!());
        assert!(list != null!());
        assert!(list_remove_entry(c_ref!(list), list) != 0);
        assert!(list == null!());
        list = generate_list();
        entry = list_nth_entry(list, 3);
        assert!(list_remove_entry(c_ref!(list), entry) != 0);
        check_list_integrity(list);
        list_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_remove_data() {
    unsafe {
        let mut entries: Array<i32, 13> = arr![89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4];
        let num_entries: u32 = entries.len() as u32;
        let mut val: i32;
        let mut list: Manual<ListEntry<Ptr<i32>>> = null!();
        let mut i: u32;

        c_for!(i = 0; i < num_entries; i += 1; {
            assert!(list_prepend(c_ref!(list), c_ref!(entries[i as usize])) != null!());
        });

        val = 0;
        assert!(list_remove_data(c_ref!(list), func!(int_equal), c_ref!(val)) == 0);

        val = 56;
        assert!(list_remove_data(c_ref!(list), func!(int_equal), c_ref!(val)) == 0);
        check_list_integrity(list);

        val = 8;
        assert!(list_remove_data(c_ref!(list), func!(int_equal), c_ref!(val)) == 1);
        assert!(list_length(list) == num_entries - 1);
        check_list_integrity(list);

        val = 4;
        assert!(list_remove_data(c_ref!(list), func!(int_equal), c_ref!(val)) == 4);
        assert!(list_length(list) == num_entries - 5);
        check_list_integrity(list);

        val = 89;
        assert!(list_remove_data(c_ref!(list), func!(int_equal), c_ref!(val)) == 1);
        assert!(list_length(list) == num_entries - 6);
        check_list_integrity(list);

        list_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_sort() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>> = null!();
        let mut entries: Array<i32, 13> = arr![89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4];
        let sorted: Array<i32, 13> = arr![4, 4, 4, 4, 8, 15, 16, 23, 30, 42, 50, 89, 99];
        let num_entries: u32 = entries.len() as u32;
        let mut i: u32;

        c_for!(i = 0; i < num_entries; i += 1; {
            assert!(list_prepend(c_ref!(list), c_ref!(entries[i as usize])) != null!());
        });

        list_sort(c_ref!(list), func!(int_compare));
        assert!(list_length(list) == num_entries);

        c_for!(i = 0; i < num_entries; i += 1; {
            let mut value: Ptr<i32>;
            value = list_nth_data(list, i);
            assert!(*value == sorted[i as usize]);
        });

        list_free(list);
        list = null!();
        list_sort(c_ref!(list), func!(int_compare));
        assert!(list == null!());
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_find_data() {
    unsafe {
        let mut entries: Array<i32, 10> = arr![89, 23, 42, 16, 15, 4, 8, 99, 50, 30];
        let num_entries: u32 = entries.len() as u32;
        let mut list: Manual<ListEntry<Ptr<i32>>>;
        let mut result: Manual<ListEntry<Ptr<i32>>>;
        let mut i: u32;
        let mut val: i32;
        let mut data: Ptr<i32>;
        list = null!();
        c_for!(i = 0; i < num_entries; i += 1; {
            assert!(list_append(c_ref!(list), c_ref!(entries[i])) != null!());
        });
        c_for!(i = 0; i < num_entries; i += 1; {
            val = entries[i];
            result = list_find_data(list, func!(int_equal), c_ref!(val));
            assert!(result != null!());
            data = list_data(result);
            assert!(*data == val);
        });
        val = 0;
        assert!(list_find_data(list, func!(int_equal), c_ref!(val)) == null!());
        val = 56;
        assert!(list_find_data(list, func!(int_equal), c_ref!(val)) == null!());
        list_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_to_array() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>>;
        let mut array: Vector<Ptr<i32>>;
        list = generate_list();
        array = list_to_array(list);
        assert!(array[0] == c_ref!(variable1));
        assert!(array[1] == c_ref!(variable2));
        assert!(array[2] == c_ref!(variable3));
        assert!(array[3] == c_ref!(variable4));
        c_free!(array);
        list_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_iterate() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>>;
        let mut iter: ListIterator<Ptr<i32>> = Default::default();
        let mut i: i32;
        let mut a: i32 = 0;
        let mut counter: i32;
        let mut data: Ptr<i32>;
        list = null!();
        c_for!(i = 0; i < 50; i += 1; {
            assert!(list_prepend(c_ref!(list), c_ref!(a)) != null!());
        });
        counter = 0;
        list_iterate(c_ref!(list), c_ref!(iter));
        list_iter_remove(c_ref!(iter));
        while list_iter_has_more(c_ref!(iter)) != 0 {
            data = list_iter_next(c_ref!(iter));
            counter += 1;
            if counter % 2 == 0 {
                list_iter_remove(c_ref!(iter));
                list_iter_remove(c_ref!(iter));
            }
        }
        assert!(list_iter_next(c_ref!(iter)) == null!());
        list_iter_remove(c_ref!(iter));
        assert!(counter == 50);
        assert!(list_length(list) == 25);
        list_free(list);
        list = null!();
        counter = 0;
        list_iterate(c_ref!(list), c_ref!(iter));
        while list_iter_has_more(c_ref!(iter)) != 0 {
            data = list_iter_next(c_ref!(iter));
            counter += 1;
        }
        assert!(counter == 0);
        test_no_memory_leak!();
    }
}
#[test]
fn test_list_iterate_bad_remove() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>>;
        let mut iter: ListIterator<Ptr<i32>> = Default::default();
        let mut values: Array<i32, 49> = arr![0; 49];
        let mut i: i32;
        let mut val: Ptr<i32>;
        list = null!();
        c_for!(i = 0; i < 49; i += 1; {
            values[i] = i;
            assert!(list_prepend(c_ref!(list), c_ref!(values[i])) != null!());
        });
        list_iterate(c_ref!(list), c_ref!(iter));
        while list_iter_has_more(c_ref!(iter)) != 0 {
            val = list_iter_next(c_ref!(iter));
            if *val % 2 == 0 {
                assert!(list_remove_data(c_ref!(list), func!(int_equal), val) != 0);
                list_iter_remove(c_ref!(iter));
            }
        }
        list_free(list);
        test_no_memory_leak!();
    }
}
