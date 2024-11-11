#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "bloom-filter.h"
#include "hash-string.h"
void test_bloom_filter_new_free(void)
{
	BloomFilter *filter;
	filter = bloom_filter_new(128, string_hash, 1);
	assert(filter != NULL);
	bloom_filter_free(filter);
	filter = bloom_filter_new(128, string_hash, 64);
	assert(filter != NULL);
	bloom_filter_free(filter);
	filter = bloom_filter_new(128, string_hash, 50000);
	assert(filter == NULL);
	alloc_test_set_limit(0);
	filter = bloom_filter_new(128, string_hash, 1);
	assert(filter == NULL);
	alloc_test_set_limit(1);
	filter = bloom_filter_new(128, string_hash, 1);
	assert(filter == NULL);
}
void test_bloom_filter_insert_query(void)
{
	BloomFilter *filter;
	filter = bloom_filter_new(128, string_hash, 4);
	assert(bloom_filter_query(filter, "test 1") == 0);
	assert(bloom_filter_query(filter, "test 2") == 0);
	bloom_filter_insert(filter, "test 1");
	bloom_filter_insert(filter, "test 2");
	assert(bloom_filter_query(filter, "test 1") != 0);
	assert(bloom_filter_query(filter, "test 2") != 0);
	bloom_filter_free(filter);
}
void test_bloom_filter_read_load(void)
{
	BloomFilter *filter1;
	BloomFilter *filter2;
	unsigned char state[16];
	filter1 = bloom_filter_new(128, string_hash, 4);
	bloom_filter_insert(filter1, "test 1");
	bloom_filter_insert(filter1, "test 2");
	bloom_filter_read(filter1, state);
	bloom_filter_free(filter1);
	filter2 = bloom_filter_new(128, string_hash, 4);
	bloom_filter_load(filter2, state);
	assert(bloom_filter_query(filter2, "test 1") != 0);
	assert(bloom_filter_query(filter2, "test 2") != 0);
	bloom_filter_free(filter2);
}
void test_bloom_filter_intersection(void)
{
	BloomFilter *filter1;
	BloomFilter *filter2;
	BloomFilter *result;
	filter1 = bloom_filter_new(128, string_hash, 4);
	bloom_filter_insert(filter1, "test 1");
	bloom_filter_insert(filter1, "test 2");
	filter2 = bloom_filter_new(128, string_hash, 4);
	bloom_filter_insert(filter2, "test 1");
	assert(bloom_filter_query(filter2, "test 2") == 0);
	result = bloom_filter_intersection(filter1, filter2);
	assert(bloom_filter_query(result, "test 1") != 0);
	assert(bloom_filter_query(result, "test 2") == 0);
	bloom_filter_free(result);
	alloc_test_set_limit(0);
	result = bloom_filter_intersection(filter1, filter2);
	assert(result == NULL);
	bloom_filter_free(filter1);
	bloom_filter_free(filter2);
}
void test_bloom_filter_union(void)
{
	BloomFilter *filter1;
	BloomFilter *filter2;
	BloomFilter *result;
	filter1 = bloom_filter_new(128, string_hash, 4);
	bloom_filter_insert(filter1, "test 1");
	filter2 = bloom_filter_new(128, string_hash, 4);
	bloom_filter_insert(filter2, "test 2");
	result = bloom_filter_union(filter1, filter2);
	assert(bloom_filter_query(result, "test 1") != 0);
	assert(bloom_filter_query(result, "test 2") != 0);
	bloom_filter_free(result);
	alloc_test_set_limit(0);
	result = bloom_filter_union(filter1, filter2);
	assert(result == NULL);
	bloom_filter_free(filter1);
	bloom_filter_free(filter2);
}
void test_bloom_filter_mismatch(void)
{
	BloomFilter *filter1;
	BloomFilter *filter2;
	filter1 = bloom_filter_new(128, string_hash, 4);
	filter2 = bloom_filter_new(64, string_hash, 4);
	assert(bloom_filter_intersection(filter1, filter2) == NULL);
	assert(bloom_filter_union(filter1, filter2) == NULL);
	bloom_filter_free(filter2);
	filter2 = bloom_filter_new(128, string_nocase_hash, 4);
	assert(bloom_filter_intersection(filter1, filter2) == NULL);
	assert(bloom_filter_union(filter1, filter2) == NULL);
	bloom_filter_free(filter2);
	filter2 = bloom_filter_new(128, string_hash, 32);
	assert(bloom_filter_intersection(filter1, filter2) == NULL);
	assert(bloom_filter_union(filter1, filter2) == NULL);
	bloom_filter_free(filter2);
	bloom_filter_free(filter1);
}
static UnitTestFunction tests[] = {
	test_bloom_filter_new_free,
	test_bloom_filter_insert_query,
	test_bloom_filter_read_load,
	test_bloom_filter_intersection,
	test_bloom_filter_union,
	test_bloom_filter_mismatch,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
