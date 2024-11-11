
use primary::arraylist::*;

pub fn arraylist_append(arraylist: &mut ArrayList, data: ArrayListValue) -> i32 {
    arraylist_insert(arraylist, arraylist.length, data)
}
#[test]
pub fn test_arraylist_append() {
    let mut arraylist = arraylist_new(0).unwrap();

    assert!(arraylist.length == 0);

    /* Append some entries */

    assert!(arraylist_append(&mut arraylist, &variable1) != 0);
    assert!(arraylist.length == 1);

    assert!(arraylist_append(&mut arraylist, &variable2) != 0);
    assert!(arraylist.length == 2);

    assert!(arraylist_append(&mut arraylist, &variable3) != 0);
    assert!(arraylist.length == 3);

    assert!(arraylist_append(&mut arraylist, &variable4) != 0);
    assert!(arraylist.length == 4);

    assert!(arraylist.data[0] == &variable1);
    assert!(arraylist.data[1] == &variable2);
    assert!(arraylist.data[2] == &variable3);
    assert!(arraylist.data[3] == &variable4);

    /* Test appending many entries */

    for i in 0..10000 {
        assert!(arraylist_append(&mut arraylist, None) != 0);
    }

    arraylist_free(Some(&mut arraylist));

    /* Test low memory scenario */

    arraylist = arraylist_new(100).unwrap();
}