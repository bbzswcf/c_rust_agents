#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "binomial-heap.h"
#include "compare-int.h"
#define NUM_TEST_VALUES 10000
int test_array[NUM_TEST_VALUES];
void test_binomial_heap_new_free(void)
{
	BinomialHeap *heap;
	int i;
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN, int_compare);
		binomial_heap_free(heap);
	}
	alloc_test_set_limit(0);
	assert(binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN, int_compare) == NULL);
}
void test_binomial_heap_insert(void)
{
	BinomialHeap *heap;
	int i;
	heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN, int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		assert(binomial_heap_insert(heap, &test_array[i]) != 0);
	}
	assert(binomial_heap_num_entries(heap) == NUM_TEST_VALUES);
	alloc_test_set_limit(0);
	assert(binomial_heap_insert(heap, &i) == 0);
	binomial_heap_free(heap);
}
void test_min_heap(void)
{
	BinomialHeap *heap;
	int *val;
	int i;
	heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN, int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		assert(binomial_heap_insert(heap, &test_array[i]) != 0);
	}
	i = -1;
	while (binomial_heap_num_entries(heap) > 0) {
		val = (int *) binomial_heap_pop(heap);
		assert(*val == i + 1);
		i = *val;
	}
	val = (int *) binomial_heap_pop(heap);
	assert(val == NULL);
	binomial_heap_free(heap);
}
void test_max_heap(void)
{
	BinomialHeap *heap;
	int *val;
	int i;
	heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MAX, int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		assert(binomial_heap_insert(heap, &test_array[i]) != 0);
	}
	i = NUM_TEST_VALUES;
	while (binomial_heap_num_entries(heap) > 0) {
		val = (int *) binomial_heap_pop(heap);
		assert(*val == i - 1);
		i = *val;
	}
	val = (int *) binomial_heap_pop(heap);
	assert(val == NULL);
	binomial_heap_free(heap);
}
#define TEST_VALUE (NUM_TEST_VALUES / 2)
static BinomialHeap *generate_heap(void)
{
	BinomialHeap *heap;
	int i;
	heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN, int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		if (i != TEST_VALUE) {
			assert(binomial_heap_insert(heap,
			                            &test_array[i]) != 0);
		}
	}
	return heap;
}
static void verify_heap(BinomialHeap *heap)
{
	unsigned int num_vals;
	int *val;
	int i;
	num_vals = binomial_heap_num_entries(heap);
	assert(num_vals == NUM_TEST_VALUES - 1);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		if (i == TEST_VALUE) {
			continue;
		}
		val = binomial_heap_pop(heap);
		assert(*val == i);
		--num_vals;
		assert(binomial_heap_num_entries(heap) == num_vals);
	}
}
static void test_insert_out_of_memory(void)
{
	BinomialHeap *heap;
	int i;
	for (i=0; i<6; ++i) {
		heap = generate_heap();
		alloc_test_set_limit(i);
		test_array[TEST_VALUE] = TEST_VALUE;
		assert(binomial_heap_insert(heap,
		                            &test_array[TEST_VALUE]) == 0);
		alloc_test_set_limit(-1);
		verify_heap(heap);
		binomial_heap_free(heap);
	}
}
void test_pop_out_of_memory(void)
{
	BinomialHeap *heap;
	int i;
	for (i=0; i<6; ++i) {
		heap = generate_heap();
		alloc_test_set_limit(i);
		assert(binomial_heap_pop(heap) == NULL);
		alloc_test_set_limit(-1);
		binomial_heap_free(heap);
	}
}
static UnitTestFunction tests[] = {
	test_binomial_heap_new_free,
	test_binomial_heap_insert,
	test_min_heap,
	test_max_heap,
	test_insert_out_of_memory,
	test_pop_out_of_memory,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
