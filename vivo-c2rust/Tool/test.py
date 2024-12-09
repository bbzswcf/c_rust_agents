from extract_component import *
from split_function import split_rust_functions_in_file
from conftest import EMPTY_PATH, TMP_EXTRACT_FILE_PATH, TMP_REMOVE_FILE_PATH, test_extract_content, test_remove_content
from c_rust import function_replace, function_restore, comment_code
from c_rust import remove_extern_declaration_for_testfile, remove_struct_and_type_declaration, remove_main_and_tests
from c_rust import make_c_project, c2rust_convert
from Agent import Agent
import pytest


# extract_extern_declaration
# extract_func_signature_from_extern_declaration
# extract_struct_declaration
# extract_static_declaration
# extract_type_declaration
# extract_function_names
# split_rust_functions_in_file

@pytest.mark.order(1)
def test_extract_extern_declaration():
    ans = """extern "C" {
    pub type _BinomialHeap;
    fn binomial_heap_new(
        heap_type: BinomialHeapType,
        compare_func: BinomialHeapCompareFunc,
    ) -> *mut BinomialHeap;
    fn binomial_heap_free(heap: *mut BinomialHeap);
    fn binomial_heap_insert(
        heap: *mut BinomialHeap,
        value: BinomialHeapValue,
    ) -> libc::c_int;
}"""
    res = extract_extern_declaration(EMPTY_PATH)
    assert res == None

    res  = extract_extern_declaration(TMP_EXTRACT_FILE_PATH)
    print (res)
    assert res == ans

@pytest.mark.order(2)
def test_extract_func_signature_from_extern_declaration():
    res = extract_func_signature_from_extern_declaration('')
    assert res == None
    
    input = """extern "C" {
    fn binomial_heap_new(
        heap_type: BinomialHeapType,
        compare_func: BinomialHeapCompareFunc,
    ) -> *mut BinomialHeap;
    fn binomial_heap_free(heap: *mut BinomialHeap);
    fn binomial_heap_insert(
        heap: *mut BinomialHeap,
        value: BinomialHeapValue,
    ) -> libc::c_int;
}"""
    res = extract_func_signature_from_extern_declaration(input)
    keys = {"binomial_heap_new", "binomial_heap_insert", "binomial_heap_free"}
    values = {
        "fn binomial_heap_new(\n        heap_type: BinomialHeapType,\n        compare_func: BinomialHeapCompareFunc,\n    ) -> *mut BinomialHeap;",
        "fn binomial_heap_free(heap: *mut BinomialHeap);",
        "fn binomial_heap_insert(\n        heap: *mut BinomialHeap,\n        value: BinomialHeapValue,\n    ) -> libc::c_int;"
    }
    assert set(res.keys()) == keys
    assert set(res.values()) == values
    print(res)

@pytest.mark.order(3)
def test_extract_struct_declaration():
    ans = ["""pub struct _ArrayList {
    pub data: *mut ArrayListValue,
    pub length: libc::c_uint,
    pub _alloced: libc::c_uint,
}"""]
    res = extract_struct_declaration(EMPTY_PATH)
    assert res == None

    res = extract_struct_declaration(TMP_EXTRACT_FILE_PATH)
    print(res)
    assert res == ans

@pytest.mark.order(4)
def test_extract_static_declaration():
    ans=[
        "pub static mut test_array: [libc::c_int; 10000] = [0; 10000];",
        "pub static mut variable1: libc::c_int = 0;"
    ]
    
    res = extract_static_declaration(EMPTY_PATH)
    assert res == None

    res  = extract_static_declaration(TMP_EXTRACT_FILE_PATH)
    print (res)
    assert res == ans

@pytest.mark.order(5)
def test_extract_type_declaration():
    ans = [
        "pub type BinomialHeapType = libc::c_uint;",
        'pub type BinomialHeapCompareFunc = Option::<\n    unsafe extern "C" fn(BinomialHeapValue, BinomialHeapValue) -> libc::c_int,\n>;'
    ]
    res = extract_type_declaration(EMPTY_PATH)
    assert res == None

    res  = extract_type_declaration(TMP_EXTRACT_FILE_PATH)
    print (res)
    assert res == ans

@pytest.mark.order(6)
def test_extract_function_names():
    ans = [
        "test_binomial_heap_new_free",
        "test_pop_out_of_memory",
    ]
    res = extract_function_names(EMPTY_PATH)
    assert res == None

    res  = extract_function_names(TMP_EXTRACT_FILE_PATH)
    print (res)
    assert res == ans

@pytest.mark.order(7)
def test_split_rust_functions_in_file():
    ans_names = ["test_binomial_heap_new_free", "test_pop_out_of_memory"]
    ans_codes = [
        'pub unsafe extern "C" fn test_binomial_heap_new_free() {\n    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;\n    let mut i: libc::c_int = 0;\n    i = 0 as libc::c_int;\n    while i < 10000 as libc::c_int {\n        heap = binomial_heap_new(\n            BINOMIAL_HEAP_TYPE_MIN,\n            Some(\n                int_compare\n                    as unsafe extern "C" fn(\n                        *mut libc::c_void,\n                        *mut libc::c_void,\n                    ) -> libc::c_int,\n            ),\n        );\n        binomial_heap_free(heap);\n        i += 1;\n        i;\n    }\n}', 
        'pub unsafe extern "C" fn test_pop_out_of_memory() {\n    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;\n    let mut i: libc::c_int = 0;\n    i = 0 as libc::c_int;\n    while i < 6 as libc::c_int {\n        heap = generate_heap();\n        i += 1;\n        i;\n    }\n}'
    ]
    res = split_rust_functions_in_file(EMPTY_PATH)
    assert res == None

    res = split_rust_functions_in_file(TMP_EXTRACT_FILE_PATH)
    res_names = [function['name'] for function in res]
    res_codes = [function['code'] for function in res]
    print(res_names)
    print(res_codes)
    assert res_names == ans_names
    assert res_codes == ans_codes

@pytest.mark.order(8)
def test_function_replace_and_restore():
    origin_fun="""pub unsafe extern "C" fn test_pop_out_of_memory() {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        heap = generate_heap();
        i += 1;
        i;
    }
}"""
    new_fun="""pub unsafe extern "C" fn int_equal(
    mut vlocation1: *mut libc::c_void,
    mut vlocation2: *mut libc::c_void,
) -> libc::c_int {
    let mut location1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut location2: *mut libc::c_int = 0 as *mut libc::c_int;
    location1 = vlocation1 as *mut libc::c_int;
    location2 = vlocation2 as *mut libc::c_int;
    return (*location1 == *location2) as libc::c_int;
}"""
    replaced_content = test_extract_content.replace(origin_fun, new_fun)

    res = function_replace(TMP_EXTRACT_FILE_PATH, new_fun, new_fun)
    assert res == False
    res = function_replace(TMP_EXTRACT_FILE_PATH, origin_fun, new_fun)
    with open(TMP_EXTRACT_FILE_PATH, 'r') as file:
        content = file.read()
    assert res == True
    assert content == replaced_content

    res = function_restore(TMP_EXTRACT_FILE_PATH, origin_fun, origin_fun)
    assert res == False
    res = function_restore(TMP_EXTRACT_FILE_PATH, origin_fun, new_fun)
    with open(TMP_EXTRACT_FILE_PATH, 'r') as file:
        content = file.read()
    assert res == True
    assert content == test_extract_content

@pytest.mark.order(9)
def test_comment_code():
    target_code="""pub unsafe extern "C" fn test_pop_out_of_memory() {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        heap = generate_heap();
        i += 1;
        i;
    }
}"""
    replaced_comment = test_extract_content.replace(f"{target_code}\n", '')
    res = comment_code(TMP_EXTRACT_FILE_PATH, "abcde")
    assert res == False

    res = comment_code(TMP_EXTRACT_FILE_PATH, target_code)
    assert res == True
    with open(TMP_EXTRACT_FILE_PATH, 'r') as file:
        content = file.read()
    assert content == replaced_comment
    with open(TMP_EXTRACT_FILE_PATH, "w") as file:
        file.write(test_extract_content)

@pytest.mark.order(10)
def test_remove_functions():
    ans="""
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
        fn arraylist_new(length: libc::c_uint) -> *mut ArrayList;
    fn arraylist_free(arraylist: *mut ArrayList);
    fn arraylist_append(arraylist: *mut ArrayList, data: ArrayListValue) -> libc::c_int;
    fn arraylist_prepend(arraylist: *mut ArrayList, data: ArrayListValue) -> libc::c_int;
    fn arraylist_remove(arraylist: *mut ArrayList, index: libc::c_uint);
    fn arraylist_remove_range(
        arraylist: *mut ArrayList,
        index: libc::c_uint,
        length: libc::c_uint,
    );
    fn arraylist_insert(
        arraylist: *mut ArrayList,
        index: libc::c_uint,
        data: ArrayListValue,
    ) -> libc::c_int;
    fn arraylist_index_of(
        arraylist: *mut ArrayList,
        callback: ArrayListEqualFunc,
        data: ArrayListValue,
    ) -> libc::c_int;
    fn arraylist_clear(arraylist: *mut ArrayList);
    fn arraylist_sort(arraylist: *mut ArrayList, compare_func: ArrayListCompareFunc);
        }
#[no_mangle]
pub unsafe extern "C" fn generate_arraylist() -> *mut ArrayList {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    let mut i: libc::c_int = 0;
    arraylist = arraylist_new(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        arraylist_append(
            arraylist,
            &mut variable1 as *mut libc::c_int as ArrayListValue,
        );
        arraylist_append(
            arraylist,
            &mut variable2 as *mut libc::c_int as ArrayListValue,
        );
        arraylist_append(
            arraylist,
            &mut variable3 as *mut libc::c_int as ArrayListValue,
        );
        arraylist_append(
            arraylist,
            &mut variable4 as *mut libc::c_int as ArrayListValue,
        );
        i += 1;
        i;
    }
    return arraylist;
}
"""
    remove_extern_declaration_for_testfile(TMP_REMOVE_FILE_PATH, [])
    remove_struct_and_type_declaration(TMP_REMOVE_FILE_PATH)
    remove_main_and_tests(TMP_REMOVE_FILE_PATH)
    with open(TMP_REMOVE_FILE_PATH, 'r') as file:
        content = file.read()
    assert content == ans

    with open(TMP_REMOVE_FILE_PATH, "w") as file:
        file.write(test_remove_content)

@pytest.mark.order(11)
def test_make_c_project():
    res = make_c_project()
    assert res == True

@pytest.mark.order(12)
def test_c2rust_convert():
    res = c2rust_convert()
    assert res == True

@pytest.mark.order(13)
def test_LLM():
    agent = Agent(
        role="LLM test",
        prompt="You are a proficient C and Rust advanced developer.",
        temperature=0.3,
        top_p=0.8
    )
    output = agent.generate_response("hello")
    assert output != ""