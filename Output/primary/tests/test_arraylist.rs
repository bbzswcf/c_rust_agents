use primary::arraylist::*;
use primary::compare_int::*;

pub type ArrayListValue = *mut ();

pub struct ArrayList {
    pub data: Vec<ArrayListValue>,
    pub length: usize,
    pub _alloced: usize,
}

pub type ArrayListEqualFunc = fn(ArrayListValue, ArrayListValue) -> i32;
pub type ArrayListCompareFunc = fn(ArrayListValue, ArrayListValue) -> i32;

pub static mut VARIABLE1: i32 = 0;
pub static mut VARIABLE2: i32 = 0;
pub static mut VARIABLE3: i32 = 0;
pub static mut VARIABLE4: i32 = 0;
pub fn generate_arraylist() -> Option<Box<ArrayList>> {
    let mut arraylist = arraylist_new(0)?;
    let mut i = 0;

    while i < 4 {
        arraylist_append(&mut arraylist, &variable1);
        arraylist_append(&mut arraylist, &variable2);
        arraylist_append(&mut arraylist, &variable3);
        arraylist_append(&mut arraylist, &variable4);
        i += 1;
    }

    Some(arraylist)
}
pub fn test_arraylist_new_free() {
    // Use a default size when given zero
    let mut arraylist = arraylist_new(0);
    assert!(arraylist.is_some());
    arraylist_free(&mut arraylist);

    // Normal allocated
    arraylist = arraylist_new(10);
    assert!(arraylist.is_some());
    arraylist_free(&mut arraylist);

    // Freeing a null arraylist works
    arraylist_free(&mut None);

    // Test low memory scenarios (failed malloc)
    alloc_test_set_limit(0);
    arraylist = arraylist_new(0);
    assert!(arraylist.is_none());

    alloc_test_set_limit(1);
    arraylist = arraylist_new(100);
    assert!(arraylist.is_none());
}