
/*

Copyright (c) 2005-2008, Simon Howard

Permission to use, copy, modify, and/or distribute this software
for any purpose with or without fee is hereby granted, provided
that the above copyright notice and this permission notice appear
in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL
WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR
CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

 */

/* Tests for the allocation testing framework. */

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

	/* Allocate a block and check that the counters increase */

}

static void test_realloc(void)
{
	void *block;
	void *block2;

	/* This block will be allocated while the other tests are run */

	block2 = malloc(1024);

	/* Allocate a block */

	block = malloc(1024);

	assert(block != NULL);
}

static void test_calloc(void)
{
	unsigned char *block;
	int i;

}

static void test_strdup(void)
{
	char *str;

}

static void test_limits(void)
{
	void *block;

	/* Test normal malloc */

	block = malloc(2048);
	assert(block != NULL);
	free(block);

	/* Test malloc with limit */

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

