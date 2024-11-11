#ifndef ALLOC_TESTING_H
#define ALLOC_TESTING_H
#ifndef ALLOC_TESTING_C
#undef malloc
#define malloc   alloc_test_malloc
#undef free
#define free     alloc_test_free
#undef realloc
#define realloc  alloc_test_realloc
#undef calloc
#define calloc   alloc_test_calloc
#undef strdup
#define strdup   alloc_test_strdup
#endif
void *alloc_test_malloc(size_t bytes);
void alloc_test_free(void *ptr);
void *alloc_test_realloc(void *ptr, size_t bytes);
void *alloc_test_calloc(size_t nmemb, size_t bytes);
char *alloc_test_strdup(const char *string);
void alloc_test_set_limit(signed int alloc_count);
size_t alloc_test_get_allocated(void);
#endif 
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#define ALLOC_TESTING_C
#include "alloc-testing.h"
#define ALLOC_TEST_MAGIC 0x72ec82d2
#define MALLOC_PATTERN 0xBAADF00D
#define FREE_PATTERN 0xDEADBEEF
typedef struct _BlockHeader BlockHeader;
struct _BlockHeader {
	unsigned int magic_number;
	size_t bytes;
};
static size_t allocated_bytes = 0;
signed int allocation_limit = -1;
static BlockHeader *alloc_test_get_header(void *ptr)
{
	BlockHeader *result;
	result = ((BlockHeader *) ptr) - 1;
	assert(result->magic_number == ALLOC_TEST_MAGIC);
	return result;
}
static void alloc_test_overwrite(void *ptr, size_t length,
                                 unsigned int pattern)
{
	unsigned char *byte_ptr;
	int pattern_seq;
	unsigned char b;
	size_t i;
	byte_ptr = ptr;
	for (i=0; i<length; ++i) {
		pattern_seq = (int) (i & 3);
		b = (unsigned char) ((pattern >> (8 * pattern_seq)) & 0xff);
		byte_ptr[i] = b;
	}
}
void *alloc_test_malloc(size_t bytes)
{
	BlockHeader *header;
	void *ptr;
	if (allocation_limit == 0) {
		return NULL;
	}
	header = malloc(sizeof(BlockHeader) + bytes);
	if (header == NULL) {
		return NULL;
	}
	header->magic_number = ALLOC_TEST_MAGIC;
	header->bytes = bytes;
	ptr = header + 1;
	alloc_test_overwrite(ptr, bytes, MALLOC_PATTERN);
	allocated_bytes += bytes;
	if (allocation_limit > 0) {
		--allocation_limit;
	}
	return header + 1;
}
void alloc_test_free(void *ptr)
{
	BlockHeader *header;
	size_t block_size;
	if (ptr == NULL) {
		return;
	}
	header = alloc_test_get_header(ptr);
	block_size = header->bytes;
	assert(allocated_bytes >= block_size);
	alloc_test_overwrite(ptr, header->bytes, FREE_PATTERN);
	header->magic_number = 0;
	free(header);
	allocated_bytes -= block_size;
}
void *alloc_test_realloc(void *ptr, size_t bytes)
{
	BlockHeader *header;
	void *new_ptr;
	size_t bytes_to_copy;
	new_ptr = alloc_test_malloc(bytes);
	if (new_ptr == NULL) {
		return NULL;
	}
	if (ptr != NULL) {
		header = alloc_test_get_header(ptr);
		bytes_to_copy = header->bytes;
		if (bytes_to_copy > bytes) {
			bytes_to_copy = bytes;
		}
		memcpy(new_ptr, ptr, bytes_to_copy);
		alloc_test_free(ptr);
	}
	return new_ptr;
}
void *alloc_test_calloc(size_t nmemb, size_t bytes)
{
	void *result;
	size_t total_bytes = nmemb * bytes;
	result = alloc_test_malloc(total_bytes);
	if (result == NULL) {
		return NULL;
	}
	memset(result, 0, total_bytes);
	return result;
}
char *alloc_test_strdup(const char *string)
{
	char *result;
	result = alloc_test_malloc(strlen(string) + 1);
	if (result == NULL) {
		return NULL;
	}
	strcpy(result, string);
	return result;
}
void alloc_test_set_limit(signed int alloc_count)
{
	allocation_limit = alloc_count;
}
size_t alloc_test_get_allocated(void)
{
	return allocated_bytes;
}
