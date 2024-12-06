pub extern "C" fn arraylist_new(mut length: libc::c_uint) -> *mut ArrayList {
    let mut new_arraylist: *mut ArrayList = std::ptr::null_mut();
    if length == 0 {
        length = 16;
    }

    new_arraylist = alloc_test_malloc(
        std::mem::size_of::<ArrayList>() as libc::c_ulong,
    ) as *mut ArrayList;

    if new_arraylist.is_null() {
        return std::ptr::null_mut();
    } 

    unsafe {
        (*new_arraylist)._alloced = length;
        (*new_arraylist).length = 0;
        (*new_arraylist).data = alloc_test_malloc(
            (length as libc::c_ulong)
                .wrapping_mul(std::mem::size_of::<ArrayListValue>() as libc::c_ulong),
        ) as *mut ArrayListValue;
    }
    if unsafe { (*new_arraylist).data.is_null() } {
        unsafe {
            alloc_test_free(new_arraylist as *mut libc::c_void);
        }
        return std::ptr::null_mut();
    }

    new_arraylist
}



pub extern "C" fn arraylist_new(mut length: libc::c_uint) -> *mut ArrayList {
    if length == 0 {
        length = 16;
    }
    let new_arraylist: *mut ArrayList = alloc_test_malloc(
        std::mem::size_of::<ArrayList>() as libc::c_ulong
    ) as *mut ArrayList;
    if new_arraylist.is_null() {
        return std::ptr::null_mut();
    }
    
    let mut boxed_arraylist = unsafe { Box::from_raw(new_arraylist) };
    
    boxed_arraylist._alloced = length;
    boxed_arraylist.length = 0;
    boxed_arraylist.data = alloc_test_malloc(
        (length as libc::c_ulong).wrapping_mul(std::mem::size_of::<ArrayListValue>() as libc::c_ulong)
    ) as *mut ArrayListValue;
    
    
    if boxed_arraylist.data.is_null() {
        alloc_test_free(new_arraylist as *mut libc::c_void);
        return std::ptr::null_mut();
    }
    
    let new_arraylist = Box::into_raw(boxed_arraylist);
    new_arraylist
}