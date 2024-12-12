use crate::bloom_filter::*;
use crate::hash_string::*;
use crate::translation_utils::*;
#[test]
fn test_bloom_filter_new_free() {
    let mut filter: Owned<BloomFilter<CStr>>;
    filter = bloom_filter_new(128, func!(string_hash), 1);
    assert!(filter != null!());
    bloom_filter_free(filter.unowned());
    filter = bloom_filter_new(128, func!(string_hash), 64);
    assert!(filter != null!());
    bloom_filter_free(filter.unowned());
    filter = bloom_filter_new(128, func!(string_hash), 50000);
    assert!(filter == null!());
}
#[test]
fn test_bloom_filter_insert_query() {
    let mut filter: Owned<BloomFilter<CStr>>;
    filter = bloom_filter_new(128, func!(string_hash), 4);
    assert!(bloom_filter_query(filter.unowned(), cstr!("test 1")) == 0);
    assert!(bloom_filter_query(filter.unowned(), cstr!("test 2")) == 0);
    bloom_filter_insert(filter.unowned(), cstr!("test 1"));
    bloom_filter_insert(filter.unowned(), cstr!("test 2"));
    assert!(bloom_filter_query(filter.unowned(), cstr!("test 1")) != 0);
    assert!(bloom_filter_query(filter.unowned(), cstr!("test 2")) != 0);
    bloom_filter_free(filter.unowned());
}
#[test]
fn test_bloom_filter_read_load() {
    let mut filter1: Owned<BloomFilter<CStr>>;
    let mut filter2: Owned<BloomFilter<CStr>>;
    let mut state: Array<u8, 16> = arr![0; 16];

    filter1 = bloom_filter_new(128, func!(string_hash), 4);
    bloom_filter_insert(filter1.unowned(), cstr!("test 1"));
    bloom_filter_insert(filter1.unowned(), cstr!("test 2"));
    bloom_filter_read(filter1.unowned(), state.unowned());
    bloom_filter_free(filter1.unowned());

    filter2 = bloom_filter_new(128, func!(string_hash), 4);
    bloom_filter_load(filter2.unowned(), state.unowned());
    assert!(bloom_filter_query(filter2.unowned(), cstr!("test 1")) != 0);
    assert!(bloom_filter_query(filter2.unowned(), cstr!("test 2")) != 0);
    bloom_filter_free(filter2.unowned());
}
#[test]
fn test_bloom_filter_intersection() {
    let mut filter1: Owned<BloomFilter<CStr>>;
    let mut filter2: Owned<BloomFilter<CStr>>;
    let mut result: Owned<BloomFilter<CStr>>;

    filter1 = bloom_filter_new(128, func!(string_hash), 4);
    bloom_filter_insert(filter1.unowned(), cstr!("test 1"));
    bloom_filter_insert(filter1.unowned(), cstr!("test 2"));

    filter2 = bloom_filter_new(128, func!(string_hash), 4);
    bloom_filter_insert(filter2.unowned(), cstr!("test 1"));
    assert!(bloom_filter_query(filter2.unowned(), cstr!("test 2")) == 0);

    result = bloom_filter_intersection(filter1.unowned(), filter2.unowned());
    assert!(bloom_filter_query(result.unowned(), cstr!("test 1")) != 0);
    assert!(bloom_filter_query(result.unowned(), cstr!("test 2")) == 0);

    bloom_filter_free(result.unowned());
}
#[test]
fn test_bloom_filter_union() {
    let mut filter1: Owned<BloomFilter<CStr>>;
    let mut filter2: Owned<BloomFilter<CStr>>;
    let mut result: Owned<BloomFilter<CStr>>;
    filter1 = bloom_filter_new(128, func!(string_hash), 4);
    bloom_filter_insert(filter1.unowned(), cstr!("test 1"));
    filter2 = bloom_filter_new(128, func!(string_hash), 4);
    bloom_filter_insert(filter2.unowned(), cstr!("test 2"));
    result = bloom_filter_union(filter1.unowned(), filter2.unowned());
    assert!(bloom_filter_query(result.unowned(), cstr!("test 1")) != 0);
    assert!(bloom_filter_query(result.unowned(), cstr!("test 2")) != 0);
    bloom_filter_free(result.unowned());
}
#[test]
fn test_bloom_filter_mismatch() {
    let mut filter1: Owned<BloomFilter<CStr>>;
    let mut filter2: Owned<BloomFilter<CStr>>;
    
    filter1 = bloom_filter_new(128, func!(string_hash), 4);
    filter2 = bloom_filter_new(64, func!(string_hash), 4);
    assert!(bloom_filter_intersection(filter1.unowned(), filter2.unowned()) == null!());
    assert!(bloom_filter_union(filter1.unowned(), filter2.unowned()) == null!());
    bloom_filter_free(filter2.unowned());

    filter2 = bloom_filter_new(128, func!(string_nocase_hash), 4);
    assert!(bloom_filter_intersection(filter1.unowned(), filter2.unowned()) == null!());
    assert!(bloom_filter_union(filter1.unowned(), filter2.unowned()) == null!());
    bloom_filter_free(filter2.unowned());

    filter2 = bloom_filter_new(128, func!(string_hash), 32);
    assert!(bloom_filter_intersection(filter1.unowned(), filter2.unowned()) == null!());
    assert!(bloom_filter_union(filter1.unowned(), filter2.unowned()) == null!());
    bloom_filter_free(filter2.unowned());
    
    bloom_filter_free(filter1.unowned());
}
