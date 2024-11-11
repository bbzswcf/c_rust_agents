#include <stdio.h>
#include <assert.h>
#include <string.h>
#include "alloc-testing.h"
#include "framework.h"
static void test_malloc_free(void)
{
	void *block, *block2, *block3, *block4;
	unsigned char *ptr;
	int i;
	assert(alloc_test_get_allocated() == 0);
	block = malloc(1024);
	assert(block != NULL);
	assert(alloc_test_get_allocated() == 1024);
	ptr = block;
	for (i=0; i<1024; ++i) {
		assert(ptr[i] != 0);
	}
	free(block);
	assert(alloc_test_get_allocated() == 0);
	alloc_test_set_limit(3);
	block = malloc(1024);
	assert(block != NULL);
	block2 = malloc(1024);
	assert(block2 != NULL);
	block3 = malloc(1024);
	assert(block3 != NULL);
	block4 = malloc(1024);
	assert(block4 == NULL);
	free(block);
	free(block2);
	free(block3);
	free(block4);
}
static void test_realloc(void)
{
	void *block;
	void *block2;
	block2 = malloc(1024);
	block = malloc(1024);
	assert(block != NULL);
	assert(alloc_test_get_allocated() == 1024 + 1024);
	block = realloc(block, 2048);
	assert(block != NULL);
	assert(alloc_test_get_allocated() == 2048 + 1024);
	block = realloc(block, 1500);
	assert(block != NULL);
	assert(alloc_test_get_allocated() == 1500 + 1024);
	free(block);
	assert(alloc_test_get_allocated() == 0 + 1024);
	block = realloc(NULL, 1024);
	assert(block != NULL);
	assert(alloc_test_get_allocated() == 1024 + 1024);
	free(block);
	free(block2);
	assert(alloc_test_get_allocated() == 0);
	block = malloc(512);
	assert(block != NULL);
	assert(alloc_test_get_allocated() == 512);
	alloc_test_set_limit(1);
	block = realloc(block, 1024);
	assert(block != NULL);
	assert(alloc_test_get_allocated() == 1024);
	assert(realloc(block, 2048) == NULL);
	assert(alloc_test_get_allocated() == 1024);
	free(block);
	assert(alloc_test_get_allocated() == 0);
	alloc_test_set_limit(1);
	block = realloc(NULL, 1024);
	assert(block != NULL);
	assert(alloc_test_get_allocated() == 1024);
	assert(realloc(NULL, 1024) == NULL);
	assert(alloc_test_get_allocated() == 1024);
	free(block);
}
static void test_calloc(void)
{
	unsigned char *block;
	int i;
	assert(alloc_test_get_allocated() == 0);
	block = calloc(16, 64);
	assert(alloc_test_get_allocated() == 1024);
	assert(block != NULL);
	for (i=0; i<1024; ++i) {
		assert(block[i] == 0);
	}
	free(block);
	assert(alloc_test_get_allocated() == 0);
	alloc_test_set_limit(1);
	block = calloc(1024, 1);
	assert(block != NULL);
	assert(alloc_test_get_allocated() == 1024);
	assert(calloc(1024, 1) == NULL);
	assert(alloc_test_get_allocated() == 1024);
	free(block);
}
static void test_strdup(void)
{
	char *str;
	assert(alloc_test_get_allocated() == 0);
	str = strdup("hello world");
	assert(str != NULL);
	assert(strcmp(str, "hello world") == 0);
	assert(alloc_test_get_allocated() == 12);
	free(str);
	assert(alloc_test_get_allocated() == 0);
	alloc_test_set_limit(1);
	str = strdup("hello world");
	assert(str != NULL);
	assert(alloc_test_get_allocated() == 12);
	assert(strdup("hello world") == NULL);
	assert(alloc_test_get_allocated() == 12);
	free(str);
}
static void test_limits(void)
{
	void *block;
	block = malloc(2048);
	assert(block != NULL);
	free(block);
	alloc_test_set_limit(1);
	block = malloc(1024);
	assert(block != NULL);
	assert(malloc(1024) == NULL);
	free(block);
	alloc_test_set_limit(-1);
	block = malloc(1024);
	assert(block != NULL);
	free(block);
}
static UnitTestFunction tests[] = {
	test_malloc_free,
	test_realloc,
	test_calloc,
	test_strdup,
	test_limits,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
