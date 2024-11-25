src_codein_1="""
unsafe extern "C" fn arraylist_enlarge(mut arraylist: *mut ArrayList) -> libc::c_int {
    let mut data: *mut ArrayListValue = 0 as *mut ArrayListValue;
    let mut newsize: libc::c_uint = 0;
    newsize = ((*arraylist)._alloced).wrapping_mul(2 as libc::c_int as libc::c_uint);
    data = alloc_test_realloc(
        (*arraylist).data as *mut libc::c_void,
        (::core::mem::size_of::<ArrayListValue>() as libc::c_ulong)
            .wrapping_mul(newsize as libc::c_ulong),
    ) as *mut ArrayListValue;
    if data.is_null() {
        return 0 as libc::c_int
    } else {
        (*arraylist).data = data;
        (*arraylist)._alloced = newsize;
        return 1 as libc::c_int;
    };
}
"""

src_codeout_1="""
extern "C" fn arraylist_enlarge(mut arraylist: *mut ArrayList) -> libc::c_int {
    let mut data: *mut ArrayListValue = std::ptr::null_mut();
    let mut newsize: libc::c_uint = 0;

    // SAFETY: The caller must ensure that `arraylist` is a valid pointer.
    unsafe {
        newsize = ((*arraylist)._alloced).wrapping_mul(2);
        data = alloc_test_realloc(
            (*arraylist).data as *mut libc::c_void,
            (std::mem::size_of::<ArrayListValue>() as libc::c_ulong)
                .wrapping_mul(newsize as libc::c_ulong),
        ) as *mut ArrayListValue;
    }

    if data.is_null() {
        return 0;
    } else {
        // SAFETY: The caller must ensure that `arraylist` is a valid pointer.
        unsafe {
            (*arraylist).data = data;
            (*arraylist)._alloced = newsize;
        }
        return 1;
    }
}
"""

src_codein_2="""
unsafe extern "C" fn hash_table_allocate_table(
    mut hash_table: *mut HashTable,
) -> libc::c_int {
    let mut new_table_size: libc::c_uint = 0;
    if (*hash_table).prime_index < hash_table_num_primes {
        new_table_size = hash_table_primes[(*hash_table).prime_index as usize];
    } else {
        new_table_size = ((*hash_table).entries)
            .wrapping_mul(10 as libc::c_int as libc::c_uint);
    }
    (*hash_table).table_size = new_table_size;
    (*hash_table)
        .table = alloc_test_calloc(
        (*hash_table).table_size as size_t,
        ::core::mem::size_of::<*mut HashTableEntry>() as libc::c_ulong,
    ) as *mut *mut HashTableEntry;
    return ((*hash_table).table != 0 as *mut libc::c_void as *mut *mut HashTableEntry)
        as libc::c_int;
}
"""
src_codeout_2="""
extern "C" fn hash_table_allocate_table(
    mut hash_table: *mut HashTable,
) -> libc::c_int {
    let mut new_table_size: libc::c_uint = 0;
    unsafe {
        if (*hash_table).prime_index < hash_table_num_primes {
            new_table_size = hash_table_primes[(*hash_table).prime_index as usize];
        } else {
            new_table_size = ((*hash_table).entries).wrapping_mul(10);
        }
        (*hash_table).table_size = new_table_size;
        (*hash_table).table = alloc_test_calloc(
            (*hash_table).table_size as size_t,
            ::core::mem::size_of::<*mut HashTableEntry>() as libc::c_ulong,
        ) as *mut *mut HashTableEntry;
        ((*hash_table).table != std::ptr::null_mut()) as libc::c_int
    }
}
"""

first_optimize_prompt="""
Optimize the following rust function without changing its parameters, return value, or functionality, remove unsafe from the function declaration, and minimize the use of unsafe blocks within the function.
Other functions called in the function should be considered unsafe.
Only convert functions, do not add context.
Only output results, no need for explanation.
{rust_code}
"""

# second_optimize_prompt="""
# Optimize the following rust function without changing its parameters, return value, or functionality.
# Remove unsafe from the function declaration(if it exists).
# Minimize the range of unsafe blocks as much as possible in the function and improve the readability of the code.
# Rust function:
# {rust_code}
# Following are static global variable declarations and extern C function declarations of this crate.
# Access and usage of them should be unsafe, other functions can be considered safe.
# Declarations:
# {static_variables}
# {extern_C}
# Only convert functions, do not add context.
# Only output results, no need for explanation.
# """

second_optimize_example_code = """
pub extern "C" fn test_arraylist_sort() {
    let mut arraylist: *mut ArrayList = std::ptr::null_mut();
    let mut entries: [libc::c_int; 13] = [
        89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4,
    ];
    let mut sorted: [libc::c_int; 13] = [
        4, 4, 4, 4, 8, 15, 16, 23, 30, 42, 50, 89, 99,
    ];
    let mut num_entries: libc::c_uint = (std::mem::size_of::<[libc::c_int; 13]>()
        as libc::c_ulong)
        .wrapping_div(std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_uint;
    let mut i: libc::c_uint = 0;

    unsafe {
        arraylist = arraylist_new(10);
        i = 0;
        while i < num_entries {
            arraylist_prepend(
                arraylist,
                &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as ArrayListValue,
            );
            i = i.wrapping_add(1);
        }
        arraylist_sort(
            arraylist,
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        assert_eq!((*arraylist).length, num_entries);
        i = 0;
        while i < num_entries {
            let mut value: *mut libc::c_int = std::ptr::null_mut();
            value = *((*arraylist).data).offset(i as isize) as *mut libc::c_int;
            assert_eq!(*value, sorted[i as usize]);
            i = i.wrapping_add(1);
        }
        arraylist_free(arraylist);
        arraylist = arraylist_new(5);
        arraylist_sort(
            arraylist,
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        assert_eq!((*arraylist).length, 0);
        arraylist_free(arraylist);
        arraylist = arraylist_new(5);
        arraylist_prepend(
            arraylist,
            &mut *entries.as_mut_ptr().offset(0) as *mut libc::c_int
                as ArrayListValue,
        );
        arraylist_sort(
            arraylist,
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        assert_eq!((*arraylist).length, 1);
        assert_eq!(
            *((*arraylist).data).offset(0),
            &mut *entries.as_mut_ptr().offset(0) as *mut libc::c_int as ArrayListValue
        );
        arraylist_free(arraylist);
    }
}
"""
second_optimize_example_static = """
pub static mut variable1: libc::c_int = 0;
pub static mut variable2: libc::c_int = 0;
pub static mut variable3: libc::c_int = 0;
pub static mut variable4: libc::c_int = 0;
"""
second_optimize_example_extern_C = """
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn run_tests(tests_0: *mut UnitTestFunction);
}
"""
second_optimize_example_output = """
```rust
pub extern "C" fn test_arraylist_sort() {
    let mut arraylist: *mut ArrayList = std::ptr::null_mut();
    let mut entries: [libc::c_int; 13] = [
        89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4,
    ];
    let mut sorted: [libc::c_int; 13] = [
        4, 4, 4, 4, 8, 15, 16, 23, 30, 42, 50, 89, 99,
    ];
    let mut num_entries: libc::c_uint = (std::mem::size_of::<[libc::c_int; 13]>() / std::mem::size_of::<libc::c_int>()) as libc::c_uint;
    let mut i: libc::c_uint = 0;
    arraylist = arraylist_new(10);
    i = 0;
    while i < num_entries {
        arraylist_prepend(
            arraylist,
            &mut entries[i as usize] as *mut libc::c_int as ArrayListValue,
        );
        i = i.wrapping_add(1);
    }
    arraylist_sort(arraylist, Some(int_compare));
    assert_eq!(unsafe { (*arraylist).length }, num_entries);
    i = 0;
    while i < num_entries {
        let mut value: *mut libc::c_int = unsafe { *((*arraylist).data).offset(i as isize) as *mut libc::c_int };
        assert_eq!(unsafe { *value }, sorted[i as usize]);
        i = i.wrapping_add(1);
    }
    arraylist_free(arraylist);
    arraylist = arraylist_new(5);
    arraylist_sort(arraylist, Some(int_compare));
    assert_eq!(unsafe { (*arraylist).length }, 0);
    arraylist_free(arraylist);
    arraylist = arraylist_new(5);
    arraylist_prepend(
        arraylist,
        &mut entries[0] as *mut libc::c_int as ArrayListValue,
    );
    arraylist_sort(arraylist, Some(int_compare));
    assert_eq!(unsafe { (*arraylist).length }, 1);
    assert_eq!(
        unsafe { *((*arraylist).data).offset(0) },
        &mut entries[0] as *mut libc::c_int as ArrayListValue
    );
    arraylist_free(arraylist);
}
```
"""
second_optimize_prompt="""
Optimize the following rust function without changing its signature and functionality.
Remove unsafe from the function declaration(if it exists).
Minimize the range of unsafe blocks as much as possible in the function and improve the readability of the code.
Rust function:
{rust_code}
Following are static global variable declarations of this module, access and usage of them should be unsafe:
{static_variables}
This is the extern C declaration of this module, other functions except for these are safe:
{extern_C}
Only convert functions, do not add context.
Only output results, no need for explanation.
"""





assert_optimize_prompt="""
Replace the __assert_fail() fucntion of following rust function with an assert macro that conforms to rust conventions.
If the function does not call __assert_fail(), do not modify the function.
Only convert functions, do not add context.
Only output results, no need for explanation.
{rust_code}
"""