#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "queue.h"
int variable1, variable2, variable3, variable4;
Queue *generate_queue(void)
{
	Queue *queue;
	int i;
	queue = queue_new();
	for (i=0; i<1000; ++i) {
		queue_push_head(queue, &variable1);
		queue_push_head(queue, &variable2);
		queue_push_head(queue, &variable3);
		queue_push_head(queue, &variable4);
	}
	return queue;
}
void test_queue_new_free(void)
{
	int i;
	Queue *queue;
	queue = queue_new();
	queue_free(queue);
	queue = queue_new();
	for (i=0; i<1000; ++i) {
		queue_push_head(queue, &variable1);
	}
	queue_free(queue);
	alloc_test_set_limit(0);
	queue = queue_new();
	assert(queue == NULL);
}
void test_queue_push_head(void)
{
	Queue *queue;
	int i;
	queue = queue_new();
	for (i=0; i<1000; ++i) {
		queue_push_head(queue, &variable1);
		queue_push_head(queue, &variable2);
		queue_push_head(queue, &variable3);
		queue_push_head(queue, &variable4);
	}
	assert(!queue_is_empty(queue));
	assert(queue_pop_tail(queue) == &variable1);
	assert(queue_pop_tail(queue) == &variable2);
	assert(queue_pop_tail(queue) == &variable3);
	assert(queue_pop_tail(queue) == &variable4);
	assert(queue_pop_head(queue) == &variable4);
	assert(queue_pop_head(queue) == &variable3);
	assert(queue_pop_head(queue) == &variable2);
	assert(queue_pop_head(queue) == &variable1);
	queue_free(queue);
	queue = queue_new();
	alloc_test_set_limit(0);
	assert(!queue_push_head(queue, &variable1));
	queue_free(queue);
}
void test_queue_pop_head(void)
{
	Queue *queue;
	queue = queue_new();
	assert(queue_pop_head(queue) == NULL);
	queue_free(queue);
	queue = generate_queue();
	while (!queue_is_empty(queue)) {
		assert(queue_pop_head(queue) == &variable4);
		assert(queue_pop_head(queue) == &variable3);
		assert(queue_pop_head(queue) == &variable2);
		assert(queue_pop_head(queue) == &variable1);
	}
	assert(queue_pop_head(queue) == NULL);
	queue_free(queue);
}
void test_queue_peek_head(void)
{
	Queue *queue;
	queue = queue_new();
	assert(queue_peek_head(queue) == NULL);
	queue_free(queue);
	queue = generate_queue();
	while (!queue_is_empty(queue)) {
		assert(queue_peek_head(queue) == &variable4);
		assert(queue_pop_head(queue) == &variable4);
		assert(queue_peek_head(queue) == &variable3);
		assert(queue_pop_head(queue) == &variable3);
		assert(queue_peek_head(queue) == &variable2);
		assert(queue_pop_head(queue) == &variable2);
		assert(queue_peek_head(queue) == &variable1);
		assert(queue_pop_head(queue) == &variable1);
	}
	assert(queue_peek_head(queue) == NULL);
	queue_free(queue);
}
void test_queue_push_tail(void)
{
	Queue *queue;
	int i;
	queue = queue_new();
	for (i=0; i<1000; ++i) {
		queue_push_tail(queue, &variable1);
		queue_push_tail(queue, &variable2);
		queue_push_tail(queue, &variable3);
		queue_push_tail(queue, &variable4);
	}
	assert(!queue_is_empty(queue));
	assert(queue_pop_head(queue) == &variable1);
	assert(queue_pop_head(queue) == &variable2);
	assert(queue_pop_head(queue) == &variable3);
	assert(queue_pop_head(queue) == &variable4);
	assert(queue_pop_tail(queue) == &variable4);
	assert(queue_pop_tail(queue) == &variable3);
	assert(queue_pop_tail(queue) == &variable2);
	assert(queue_pop_tail(queue) == &variable1);
	queue_free(queue);
	queue = queue_new();
	alloc_test_set_limit(0);
	assert(!queue_push_tail(queue, &variable1));
	queue_free(queue);
}
void test_queue_pop_tail(void)
{
	Queue *queue;
	queue = queue_new();
	assert(queue_pop_tail(queue) == NULL);
	queue_free(queue);
	queue = generate_queue();
	while (!queue_is_empty(queue)) {
		assert(queue_pop_tail(queue) == &variable1);
		assert(queue_pop_tail(queue) == &variable2);
		assert(queue_pop_tail(queue) == &variable3);
		assert(queue_pop_tail(queue) == &variable4);
	}
	assert(queue_pop_tail(queue) == NULL);
	queue_free(queue);
}
void test_queue_peek_tail(void)
{
	Queue *queue;
	queue = queue_new();
	assert(queue_peek_tail(queue) == NULL);
	queue_free(queue);
	queue = generate_queue();
	while (!queue_is_empty(queue)) {
		assert(queue_peek_tail(queue) == &variable1);
		assert(queue_pop_tail(queue) == &variable1);
		assert(queue_peek_tail(queue) == &variable2);
		assert(queue_pop_tail(queue) == &variable2);
		assert(queue_peek_tail(queue) == &variable3);
		assert(queue_pop_tail(queue) == &variable3);
		assert(queue_peek_tail(queue) == &variable4);
		assert(queue_pop_tail(queue) == &variable4);
	}
	assert(queue_peek_tail(queue) == NULL);
	queue_free(queue);
}
void test_queue_is_empty(void)
{
	Queue *queue;
	queue = queue_new();
	assert(queue_is_empty(queue));
	queue_push_head(queue, &variable1);
	assert(!queue_is_empty(queue));
	queue_pop_head(queue);
	assert(queue_is_empty(queue));
	queue_push_tail(queue, &variable1);
	assert(!queue_is_empty(queue));
	queue_pop_tail(queue);
	assert(queue_is_empty(queue));
	queue_free(queue);
}
static UnitTestFunction tests[] = {
	test_queue_new_free,
	test_queue_push_head,
	test_queue_pop_head,
	test_queue_peek_head,
	test_queue_push_tail,
	test_queue_pop_tail,
	test_queue_peek_tail,
	test_queue_is_empty,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
