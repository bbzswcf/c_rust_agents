#include <stdlib.h>
#include <assert.h>
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

	/* Test low memory scenario */

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

	/* Push a load of values onto the heap */

	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		assert(binary_heap_insert(heap, &test_array[i]) != 0);
	}

	/* Pop values off the heap and check they are in order */

	i = -1;
	while (binary_heap_num_entries(heap) > 0) {
		val = (int *) binary_heap_pop(heap);

		assert(*val == i + 1);
		i = *val;
	}

	/* Test popping from an empty heap */

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

	/* Push a load of values onto the heap */

	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		assert(binary_heap_insert(heap, &test_array[i]) != 0);
	}

	/* Pop values off the heap and check they are in order */

	i = NUM_TEST_VALUES;
	while (binary_heap_num_entries(heap) > 0) {
		val = (int *) binary_heap_pop(heap);

		assert(*val == i - 1);
		i = *val;
	}

	binary_heap_free(heap);
}

int main(int argc, char *argv[])
{
	return 0;
}
