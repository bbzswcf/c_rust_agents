use crate::slist::*;
use crate::compare_int::*;
use crate::translation_utils::*;static mut variable1: i32 = 50;
static mut variable2: i32 = 0;
static mut variable3: i32 = 0;
static mut variable4: i32 = 0;
fn generate_slist() -> Manual<SListEntry<Ptr<i32>>> {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>> = null!();
        assert!(slist_append(c_ref!(list), c_ref!(variable1)) != null!());
        assert!(slist_append(c_ref!(list), c_ref!(variable2)) != null!());
        assert!(slist_append(c_ref!(list), c_ref!(variable3)) != null!());
        assert!(slist_append(c_ref!(list), c_ref!(variable4)) != null!());
        return list;
    }
}
#[test]
fn test_slist_append() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>> = null!();
        assert!(slist_append(c_ref!(list), c_ref!(variable1)) != null!());
        assert!(slist_append(c_ref!(list), c_ref!(variable2)) != null!());
        assert!(slist_append(c_ref!(list), c_ref!(variable3)) != null!());
        assert!(slist_append(c_ref!(list), c_ref!(variable4)) != null!());
        assert!(slist_length(list) == 4);
        assert!(slist_nth_data(list, 0) == c_ref!(variable1));
        assert!(slist_nth_data(list, 1) == c_ref!(variable2));
        assert!(slist_nth_data(list, 2) == c_ref!(variable3));
        assert!(slist_nth_data(list, 3) == c_ref!(variable4));
        slist_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_prepend() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>> = null!();
        assert!(slist_prepend(c_ref!(list), c_ref!(variable1)) != null!());
        assert!(slist_prepend(c_ref!(list), c_ref!(variable2)) != null!());
        assert!(slist_prepend(c_ref!(list), c_ref!(variable3)) != null!());
        assert!(slist_prepend(c_ref!(list), c_ref!(variable4)) != null!());
        assert!(slist_nth_data(list, 0) == c_ref!(variable4));
        assert!(slist_nth_data(list, 1) == c_ref!(variable3));
        assert!(slist_nth_data(list, 2) == c_ref!(variable2));
        assert!(slist_nth_data(list, 3) == c_ref!(variable1));
        slist_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_free() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        list = generate_slist();
        slist_free(list);
        slist_free::<Ptr<i32>>(null!());
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_next() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        let mut rover: Manual<SListEntry<Ptr<i32>>>;
        list = generate_slist();
        rover = list;
        assert!(slist_data(rover) == c_ref!(variable1));
        rover = slist_next(rover);
        assert!(slist_data(rover) == c_ref!(variable2));
        rover = slist_next(rover);
        assert!(slist_data(rover) == c_ref!(variable3));
        rover = slist_next(rover);
        assert!(slist_data(rover) == c_ref!(variable4));
        rover = slist_next(rover);
        assert!(rover == null!());
        slist_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_nth_entry() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        let mut entry: Manual<SListEntry<Ptr<i32>>>;
        list = generate_slist();
        entry = slist_nth_entry(list, 0);
        assert!(slist_data(entry) == c_ref!(variable1));
        entry = slist_nth_entry(list, 1);
        assert!(slist_data(entry) == c_ref!(variable2));
        entry = slist_nth_entry(list, 2);
        assert!(slist_data(entry) == c_ref!(variable3));
        entry = slist_nth_entry(list, 3);
        assert!(slist_data(entry) == c_ref!(variable4));
        entry = slist_nth_entry(list, 4);
        assert!(entry == null!());
        entry = slist_nth_entry(list, 400);
        assert!(entry == null!());
        slist_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_nth_data() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        list = generate_slist();
        assert!(slist_nth_data(list, 0) == c_ref!(variable1));
        assert!(slist_nth_data(list, 1) == c_ref!(variable2));
        assert!(slist_nth_data(list, 2) == c_ref!(variable3));
        assert!(slist_nth_data(list, 3) == c_ref!(variable4));
        assert!(slist_nth_data(list, 4) == null!());
        assert!(slist_nth_data(list, 400) == null!());
        slist_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_length() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        list = generate_slist();
        assert!(slist_length(list) == 4);
        slist_prepend(c_ref!(list), c_ref!(variable1));
        assert!(slist_length(list) == 5);
        assert!(slist_length::<Ptr<i32>>(null!()) == 0);
        slist_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_remove_entry() {
    unsafe {
        let mut empty_list: Manual<SListEntry<Ptr<i32>>> = null!();
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        let mut entry: Manual<SListEntry<Ptr<i32>>>;
        list = generate_slist();
        entry = slist_nth_entry(list, 2);
        assert!(slist_remove_entry(c_ref!(list), entry) != 0);
        assert!(slist_length(list) == 3);
        entry = slist_nth_entry(list, 0);
        assert!(slist_remove_entry(c_ref!(list), entry) != 0);
        assert!(slist_length(list) == 2);
        assert!(slist_remove_entry(c_ref!(list), entry) == 0);
        assert!(slist_remove_entry(c_ref!(list), null!()) == 0);
        assert!(slist_remove_entry(c_ref!(empty_list), null!()) == 0);
        slist_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_remove_data() {
    unsafe {
        let mut entries: Array<i32, 13> = arr![89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4];
        let num_entries: u32 = entries.len() as u32;
        let mut val: i32;
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        let mut i: u32;
        list = null!();
        c_for!(i = 0; i < num_entries; i += 1; {
            slist_prepend(c_ref!(list), c_ref!(entries[i]));
        });
        val = 0;
        assert!(slist_remove_data(c_ref!(list), func!(int_equal), c_ref!(val)) == 0);
        val = 56;
        assert!(slist_remove_data(c_ref!(list), func!(int_equal), c_ref!(val)) == 0);
        val = 8;
        assert!(slist_remove_data(c_ref!(list), func!(int_equal), c_ref!(val)) == 1);
        assert!(slist_length(list) == num_entries - 1);
        val = 4;
        assert!(slist_remove_data(c_ref!(list), func!(int_equal), c_ref!(val)) == 4);
        assert!(slist_length(list) == num_entries - 5);
        val = 89;
        assert!(slist_remove_data(c_ref!(list), func!(int_equal), c_ref!(val)) == 1);
        assert!(slist_length(list) == num_entries - 6);
        slist_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_sort() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        let mut entries: Array<i32, 13> = arr![89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4];
        let sorted: Array<i32, 13> = arr![4, 4, 4, 4, 8, 15, 16, 23, 30, 42, 50, 89, 99];
        let num_entries: u32 = entries.len() as u32;
        let mut i: u32;
        list = null!();
        c_for!(i = 0; i < num_entries; i += 1; {
            slist_prepend(c_ref!(list), c_ref!(entries[i]));
        });
        slist_sort(c_ref!(list), func!(int_compare));
        assert!(slist_length(list) == num_entries);
        c_for!(i = 0; i < num_entries; i += 1; {
            let mut value: Ptr<i32>;
            value = slist_nth_data(list, i);
            assert!(*value == sorted[i]);
        });
        slist_free(list);
        list = null!();
        slist_sort(c_ref!(list), func!(int_compare));
        assert!(list == null!());
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_find_data() {
    unsafe {
        let mut entries: Array<i32, 10> = arr![89, 23, 42, 16, 15, 4, 8, 99, 50, 30];
        let num_entries: u32 = entries.len() as u32;
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        let mut result: Manual<SListEntry<Ptr<i32>>>;
        let mut i: u32;
        let mut val: i32;
        let mut data: Ptr<i32>;
        list = null!();
        c_for!(i = 0; i < num_entries; i += 1; {
            slist_append(c_ref!(list), c_ref!(entries[i]));
        });
        c_for!(i = 0; i < num_entries; i += 1; {
            val = entries[i];
            result = slist_find_data(list, func!(int_equal), c_ref!(val));
            assert!(result != null!());
            data = slist_data(result);
            assert!(*data == val);
        });
        val = 0;
        assert!(slist_find_data(list, func!(int_equal), c_ref!(val)) == null!());
        val = 56;
        assert!(slist_find_data(list, func!(int_equal), c_ref!(val)) == null!());
        slist_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_to_array() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        let mut array: Vector<Ptr<i32>>;
        list = generate_slist();
        array = slist_to_array(list);
        assert!(array[0] == c_ref!(variable1));
        assert!(array[1] == c_ref!(variable2));
        assert!(array[2] == c_ref!(variable3));
        assert!(array[3] == c_ref!(variable4));
        c_free!(array);
        slist_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_iterate() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        let mut iter: SListIterator<Ptr<i32>> = Default::default();
        let mut data: Ptr<i32>;
        let mut a: i32 = 0;
        let mut i: i32;
        let mut counter: i32;
        list = null!();
        c_for!(i = 0; i < 50; i += 1; {
            slist_prepend(c_ref!(list), c_ref!(a));
        });
        counter = 0;
        slist_iterate(c_ref!(list), c_ref!(iter));
        slist_iter_remove(c_ref!(iter));
        while slist_iter_has_more(c_ref!(iter)) != 0 {
            data = slist_iter_next(c_ref!(iter));
            counter += 1;
            if counter % 2 == 0 {
                slist_iter_remove(c_ref!(iter));
                slist_iter_remove(c_ref!(iter));
            }
        }
        assert!(slist_iter_next(c_ref!(iter)) == slist_null!());
        slist_iter_remove(c_ref!(iter));
        assert!(counter == 50);
        assert!(slist_length(list) == 25);
        slist_free(list);
        list = null!();
        counter = 0;
        slist_iterate(c_ref!(list), c_ref!(iter));
        while slist_iter_has_more(c_ref!(iter)) != 0 {
            data = slist_iter_next(c_ref!(iter));
            counter += 1;
            if counter % 2 == 0 {
                slist_iter_remove(c_ref!(iter));
            }
        }
        assert!(counter == 0);
        test_no_memory_leak!();
    }
}
#[test]
fn test_slist_iterate_bad_remove() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>>;
        let mut iter: SListIterator<Ptr<i32>> = Default::default();
        let mut values: Array<i32, 49> = arr![0; 49];
        let mut i: i32;
        let mut val: Ptr<i32>;
        list = null!();
        c_for!(i = 0; i < 49; i += 1; {
            values[i] = i;
            assert!(slist_prepend(c_ref!(list), c_ref!(values[i])) != null!());
        });
        slist_iterate(c_ref!(list), c_ref!(iter));
        while slist_iter_has_more(c_ref!(iter)) != 0 {
            val = slist_iter_next(c_ref!(iter));
            if *val % 2 == 0 {
                assert!(slist_remove_data(c_ref!(list), func!(int_equal), val) != 0);
                slist_iter_remove(c_ref!(iter));
            }
        }
        slist_free(list);
        test_no_memory_leak!();
    }
}
