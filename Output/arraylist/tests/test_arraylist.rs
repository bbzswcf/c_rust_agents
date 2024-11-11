use primary::arraylist::*;

// pub fn test_arraylist_new_free() {
//     let arraylist = arraylist_new(0);
//     assert!(arraylist.is_some());
//     arraylist_free(arraylist);

//     let arraylist = arraylist_new(10);
//     assert!(arraylist.is_some());
//     arraylist_free(arraylist);

//     arraylist_free(None);
// }

fn test_arraylist_new_free() {
    let arraylist = arraylist_new(0);
    assert!(arraylist.is_some());
    arraylist_free(arraylist);

    let arraylist = arraylist_new(10);
    assert!(arraylist.is_some());
    arraylist_free(arraylist);

    arraylist_free(None);

    // Test low memory scenarios (failed malloc)
    // alloc_test_set_limit(0);
    let arraylist = arraylist_new(0);
    assert!(arraylist.is_none());

    // alloc_test_set_limit(1);
    let arraylist = arraylist_new(100);
    assert!(arraylist.is_none());
}