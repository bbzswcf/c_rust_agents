#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "binary-heap.h"
#include "compare-int.h"
#define NUM_TEST_VALUES 10000
int test_array[NUM_TEST_VALUES];
void test_binary_heap_new_free(void)
{
	BinaryHeap *heap;
	int i;
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		heap = binary_heap_new(BINARY_HEAP_TYPE_MIN, int_compare);
		binary_heap_free(heap);
	}
	alloc_test_set_limit(0);
	heap = binary_heap_new(BINARY_HEAP_TYPE_MIN, int_compare);
	assert(heap == NULL);
	alloc_test_set_limit(1);
	heap = binary_heap_new(BINARY_HEAP_TYPE_MIN, int_compare);
	assert(heap == NULL);
}
void test_binary_heap_insert(void)
{
	BinaryHeap *heap;
	int i;
	heap = binary_heap_new(BINARY_HEAP_TYPE_MIN, int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		assert(binary_heap_insert(heap, &test_array[i]) != 0);
	}
	assert(binary_heap_num_entries(heap) == NUM_TEST_VALUES);
	binary_heap_free(heap);
}
void test_min_heap(void)
{
	BinaryHeap *heap;
	int *val;
	int i;
	heap = binary_heap_new(BINARY_HEAP_TYPE_MIN, int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		assert(binary_heap_insert(heap, &test_array[i]) != 0);
	}
	i = -1;
	while (binary_heap_num_entries(heap) > 0) {
		val = (int *) binary_heap_pop(heap);
		assert(*val == i + 1);
		i = *val;
	}
	assert(binary_heap_num_entries(heap) == 0);
	assert(binary_heap_pop(heap) == BINARY_HEAP_NULL);
	binary_heap_free(heap);
}
void test_max_heap(void)
{
	BinaryHeap *heap;
	int *val;
	int i;
	heap = binary_heap_new(BINARY_HEAP_TYPE_MAX, int_compare);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		assert(binary_heap_insert(heap, &test_array[i]) != 0);
	}
	i = NUM_TEST_VALUES;
	while (binary_heap_num_entries(heap) > 0) {
		val = (int *) binary_heap_pop(heap);
		assert(*val == i - 1);
		i = *val;
	}
	binary_heap_free(heap);
}
void test_out_of_memory(void)
{
	BinaryHeap *heap;
	int *value;
	int values[] = {
		15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
	};
	int i;
	heap = binary_heap_new(BINARY_HEAP_TYPE_MIN, int_compare);
	alloc_test_set_limit(0);
	for (i=0; i<16; ++i) {
		assert(binary_heap_insert(heap, &values[i]) != 0);
	}
	assert(binary_heap_num_entries(heap) == 16);
	for (i=0; i<16; ++i) {
		assert(binary_heap_insert(heap, &values[i]) == 0);
		assert(binary_heap_num_entries(heap) == 16);
	}
	for (i=0; i<16; ++i) {
		value = binary_heap_pop(heap);
		assert(*value == i);
	}
	assert(binary_heap_num_entries(heap) == 0);
	binary_heap_free(heap);
}
static UnitTestFunction tests[] = {
	test_binary_heap_new_free,
	test_binary_heap_insert,
	test_min_heap,
	test_max_heap,
	test_out_of_memory,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
