use crate::arraylist::*;
use crate::compare_int::*;
use crate::translation_utils::*;static mut variable1: i32 = 0;
static mut variable2: i32 = 0;
static mut variable3: i32 = 0;
static mut variable4: i32 = 0;
fn generate_arraylist() -> Owned<ArrayList<Ptr<i32>>> {
    unsafe {
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;
        let mut i;
        arraylist = arraylist_new(0);
        c_for!(i = 0; i < 4; i += 1; {
            arraylist_append(arraylist.unowned(), c_ref!(variable1));
            arraylist_append(arraylist.unowned(), c_ref!(variable2));
            arraylist_append(arraylist.unowned(), c_ref!(variable3));
            arraylist_append(arraylist.unowned(), c_ref!(variable4));
        });
        arraylist
    }
}
#[test]
fn test_arraylist_new_free() {
    unsafe {
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;
        arraylist = arraylist_new(0);
        assert!(arraylist != null!());
        arraylist_free(arraylist.unowned());
        arraylist = arraylist_new(10);
        assert!(arraylist != null!());
        arraylist_free(arraylist.unowned());
        arraylist_free::<Ptr<i32>>(null!());
    }
}
#[test]
fn test_arraylist_append() {
    unsafe {
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;
        let mut i;
        arraylist = arraylist_new(0);
        assert!(arraylist.length == 0);
        assert!(arraylist_append(arraylist.unowned(), c_ref!(variable1)) != 0);
        assert!(arraylist.length == 1);
        assert!(arraylist_append(arraylist.unowned(), c_ref!(variable2)) != 0);
        assert!(arraylist.length == 2);
        assert!(arraylist_append(arraylist.unowned(), c_ref!(variable3)) != 0);
        assert!(arraylist.length == 3);
        assert!(arraylist_append(arraylist.unowned(), c_ref!(variable4)) != 0);
        assert!(arraylist.length == 4);
        assert!(arraylist.data[0] == c_ref!(variable1));
        assert!(arraylist.data[1] == c_ref!(variable2));
        assert!(arraylist.data[2] == c_ref!(variable3));
        assert!(arraylist.data[3] == c_ref!(variable4));
        c_for!(i = 0; i < 10000; i += 1; {
            assert!(arraylist_append(arraylist.unowned(), null!()) != 0);
        });
        arraylist_free(arraylist.unowned());
        arraylist = arraylist_new(100);
    }
}
#[test]
fn test_arraylist_prepend() {
    unsafe {
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;
        let mut i;
        arraylist = arraylist_new(0);
        assert!(arraylist.length == 0);
        assert!(arraylist_prepend(arraylist.unowned(), c_ref!(variable1)) != 0);
        assert!(arraylist.length == 1);
        assert!(arraylist_prepend(arraylist.unowned(), c_ref!(variable2)) != 0);
        assert!(arraylist.length == 2);
        assert!(arraylist_prepend(arraylist.unowned(), c_ref!(variable3)) != 0);
        assert!(arraylist.length == 3);
        assert!(arraylist_prepend(arraylist.unowned(), c_ref!(variable4)) != 0);
        assert!(arraylist.length == 4);
        assert!(arraylist.data[0] == c_ref!(variable4));
        assert!(arraylist.data[1] == c_ref!(variable3));
        assert!(arraylist.data[2] == c_ref!(variable2));
        assert!(arraylist.data[3] == c_ref!(variable1));
        c_for!(i = 0; i < 10000; i += 1; {
            assert!(arraylist_prepend(arraylist.unowned(), null!()) != 0);
        });
        arraylist_free(arraylist.unowned());
        arraylist = arraylist_new(100);
    }
}
#[test]
fn test_arraylist_insert() {
    unsafe {
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;
        let mut i;
        arraylist = generate_arraylist();
        assert!(arraylist.length == 16);
        assert!(arraylist_insert(arraylist.unowned(), 17, c_ref!(variable1)) == 0);
        assert!(arraylist.length == 16);
        assert!(arraylist.length == 16);
        assert!(arraylist.data[4] == c_ref!(variable1));
        assert!(arraylist.data[5] == c_ref!(variable2));
        assert!(arraylist.data[6] == c_ref!(variable3));
        assert!(arraylist_insert(arraylist.unowned(), 5, c_ref!(variable4)) != 0);
        assert!(arraylist.length == 17);
        assert!(arraylist.data[4] == c_ref!(variable1));
        assert!(arraylist.data[5] == c_ref!(variable4));
        assert!(arraylist.data[6] == c_ref!(variable2));
        assert!(arraylist.data[7] == c_ref!(variable3));
        assert!(arraylist.data[0] == c_ref!(variable1));
        assert!(arraylist.data[1] == c_ref!(variable2));
        assert!(arraylist.data[2] == c_ref!(variable3));
        assert!(arraylist_insert(arraylist.unowned(), 0, c_ref!(variable4)) != 0);
        assert!(arraylist.length == 18);
        assert!(arraylist.data[0] == c_ref!(variable4));
        assert!(arraylist.data[1] == c_ref!(variable1));
        assert!(arraylist.data[2] == c_ref!(variable2));
        assert!(arraylist.data[3] == c_ref!(variable3));
        assert!(arraylist.data[15] == c_ref!(variable2));
        assert!(arraylist.data[16] == c_ref!(variable3));
        assert!(arraylist.data[17] == c_ref!(variable4));
        assert!(arraylist_insert(arraylist.unowned(), 18, c_ref!(variable1)) != 0);
        assert!(arraylist.length == 19);
        assert!(arraylist.data[15] == c_ref!(variable2));
        assert!(arraylist.data[16] == c_ref!(variable3));
        assert!(arraylist.data[17] == c_ref!(variable4));
        assert!(arraylist.data[18] == c_ref!(variable1));
        c_for!(i = 0; i < 10000; i += 1; {
            arraylist_insert(arraylist.unowned(), 10, c_ref!(variable1));
        });
        arraylist_free(arraylist.unowned());
    }
}
#[test]
fn test_arraylist_remove_range() {
    unsafe {
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;
        arraylist = generate_arraylist();
        assert!(arraylist.length == 16);
        assert!(arraylist.data[3] == c_ref!(variable4));
        assert!(arraylist.data[4] == c_ref!(variable1));
        assert!(arraylist.data[5] == c_ref!(variable2));
        assert!(arraylist.data[6] == c_ref!(variable3));
        arraylist_remove_range(arraylist.unowned(), 4, 3);
        assert!(arraylist.length == 13);
        assert!(arraylist.data[3] == c_ref!(variable4));
        assert!(arraylist.data[4] == c_ref!(variable4));
        assert!(arraylist.data[5] == c_ref!(variable1));
        assert!(arraylist.data[6] == c_ref!(variable2));
        arraylist_remove_range(arraylist.unowned(), 10, 10);
        arraylist_remove_range(arraylist.unowned(), 0, 16);
        assert!(arraylist.length == 13);
        arraylist_free(arraylist.unowned());
    }
}
#[test]
fn test_arraylist_remove() {
    unsafe {
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;
        arraylist = generate_arraylist();
        assert!(arraylist.length == 16);
        assert!(arraylist.data[3] == c_ref!(variable4));
        assert!(arraylist.data[4] == c_ref!(variable1));
        assert!(arraylist.data[5] == c_ref!(variable2));
        assert!(arraylist.data[6] == c_ref!(variable3));
        arraylist_remove(arraylist.unowned(), 4);
        assert!(arraylist.length == 15);
        assert!(arraylist.data[3] == c_ref!(variable4));
        assert!(arraylist.data[4] == c_ref!(variable2));
        assert!(arraylist.data[5] == c_ref!(variable3));
        assert!(arraylist.data[6] == c_ref!(variable4));
        arraylist_remove(arraylist.unowned(), 15);
        assert!(arraylist.length == 15);
        arraylist_free(arraylist.unowned());
    }
}
#[test]
fn test_arraylist_index_of() {
    unsafe {
        let mut entries: Array<i32, 10> = arr![89, 4, 23, 42, 16, 15, 8, 99, 50, 30];
        let num_entries: i32;
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;
        let mut i: i32;
        let mut index: i32;
        let mut val: i32;

        num_entries = entries.len() as i32;
        arraylist = arraylist_new(0);

        c_for!(i = 0; i < num_entries; i += 1; {
            arraylist_append(arraylist.unowned(), c_ref!(entries[i]));
        });

        c_for!(i = 0; i < num_entries; i += 1; {
            val = entries[i];
            index = arraylist_index_of(arraylist.unowned(), func!(int_equal), c_ref!(val));
            assert!(index == i);
        });

        val = 0;
        assert!(arraylist_index_of(arraylist.unowned(), func!(int_equal), c_ref!(val)) < 0);

        val = 57;
        assert!(arraylist_index_of(arraylist.unowned(), func!(int_equal), c_ref!(val)) < 0);

        arraylist_free(arraylist.unowned());
    }
}
#[test]
fn test_arraylist_clear() {
    unsafe {
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;
        arraylist = arraylist_new(0);
        arraylist_clear(arraylist.unowned());
        assert!(arraylist.length == 0);
        arraylist_append(arraylist.unowned(), c_ref!(variable1));
        arraylist_append(arraylist.unowned(), c_ref!(variable2));
        arraylist_append(arraylist.unowned(), c_ref!(variable3));
        arraylist_append(arraylist.unowned(), c_ref!(variable4));
        arraylist_clear(arraylist.unowned());
        assert!(arraylist.length == 0);
        arraylist_free(arraylist.unowned());
    }
}
#[test]
fn test_arraylist_sort() {
    unsafe {
        let mut entries: Array<i32, 13> = arr![89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4];
        let mut sorted: Array<i32, 13> = arr![4, 4, 4, 4, 8, 15, 16, 23, 30, 42, 50, 89, 99];
        let num_entries: u32 = entries.len() as u32;
        let mut i: u32;
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;

        arraylist = arraylist_new(10);
        c_for!(i = 0; i < num_entries; i += 1; {
            arraylist_prepend(arraylist.unowned(), c_ref!(entries[i as usize]));
        });
        arraylist_sort(arraylist.unowned(), func!(int_compare));
        assert!(arraylist.length == num_entries);
        c_for!(i = 0; i < num_entries; i += 1; {
            let mut value: i32;
            value = *arraylist.data[i as usize];
            assert!(value == sorted[i as usize]);
        });
        arraylist_free(arraylist.unowned());

        arraylist = arraylist_new(5);
        arraylist_sort(arraylist.unowned(), func!(int_compare));
        assert!(arraylist.length == 0);
        arraylist_free(arraylist.unowned());

        arraylist = arraylist_new(5);
        arraylist_prepend(arraylist.unowned(), c_ref!(entries[0]));
        arraylist_sort(arraylist.unowned(), func!(int_compare));
        assert!(arraylist.length == 1);
        assert!(arraylist.data[0] == c_ref!(entries[0]));
        arraylist_free(arraylist.unowned());
    }
}
