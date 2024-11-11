// pub struct ArrayList {
//     data: Vec<Option<Box<dyn std::any::Any>>>,
//     length: usize,
//     _alloced: usize,
// }


// pub fn arraylist_new(length: usize) -> Option<Box<ArrayList>> {
//     let length = if length == 0 { 16 } else { length };
//     let data = Vec::with_capacity(length);
//     let arraylist = Box::new(ArrayList {
//         data,
//         length: 0,
//         _alloced: length,
//     });
//     Some(arraylist)
// }

// pub fn arraylist_free(_arraylist: Option<Box<ArrayList>>) {}



use std::ptr;

type ArrayListValue = *mut ();

pub struct ArrayList {
    data: Vec<ArrayListValue>,
    length: usize,
    _alloced: usize,
}

pub fn arraylist_new(length: usize) -> Option<Box<ArrayList>> {
    let length = if length == 0 { 16 } else { length };

    let data = Vec::with_capacity(length);
    let new_arraylist = Box::new(ArrayList {
        data,
        length: 0,
        _alloced: length,
    });

    Some(new_arraylist)
}

pub fn arraylist_free(_arraylist: Option<Box<ArrayList>>) {
    // Automatically managed by Rust
}