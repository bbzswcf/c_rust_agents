#include <stdlib.h>
#include <assert.h>
#include "binomial-heap.h"
#include "compare-int.h"
#define NUM_TEST_VALUES 10000

int test_array[NUM_TEST_VALUES];
#define TEST_VALUE (NUM_TEST_VALUES / 2)

void test_binomial_heap_new_free(void)
{
	BinomialHeap *heap;
	int i;

	for (i=0; i<NUM_TEST_VALUES; ++i) {
		heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN, int_compare);
		binomial_heap_free(heap);
	}

	/* Test for out of memory */

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

	/* Test for out of memory */

	}
void test_min_heap(void)
{
	BinomialHeap *heap;
	int *val;
	int i;

	heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN, int_compare);

	/* Push a load of values onto the heap */

	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		assert(binomial_heap_insert(heap, &test_array[i]) != 0);
	}

	/* Pop values off the heap and check they are in order */

	i = -1;
	while (binomial_heap_num_entries(heap) > 0) {
		val = (int *) binomial_heap_pop(heap);

		assert(*val == i + 1);
		i = *val;
	}

	/* Test pop on an empty heap */

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

	/* Push a load of values onto the heap */

	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		assert(binomial_heap_insert(heap, &test_array[i]) != 0);
	}

	/* Pop values off the heap and check they are in order */

	i = NUM_TEST_VALUES;
	while (binomial_heap_num_entries(heap) > 0) {
		val = (int *) binomial_heap_pop(heap);

		assert(*val == i - 1);
		i = *val;
	}

	/* Test pop on an empty heap */

	val = (int *) binomial_heap_pop(heap);
	assert(val == NULL);

	binomial_heap_free(heap);
}
static BinomialHeap *generate_heap(void)
{
	BinomialHeap *heap;
	int i;

	heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN, int_compare);

	/* Push a load of values onto the heap */

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

		/* Pop off the next value and check it */

		val = binomial_heap_pop(heap);
		assert(*val == i);

		/* Decrement num values counter */

		--num_vals;
		assert(binomial_heap_num_entries(heap) == num_vals);
	}
}

int main(int argc, char *argv[])
{
	return 0;
}
