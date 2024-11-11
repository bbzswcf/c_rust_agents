#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
typedef void *ArrayListValue;

struct _ArrayList {

	/** Entries in the array */

	ArrayListValue *data;

	/** Length of the array */

	unsigned int length;

	/** Private data and should not be accessed */

	unsigned int _alloced;
};

typedef struct _ArrayList ArrayList;

ArrayList *arraylist_new(unsigned int length)
{
	ArrayList *new_arraylist;

	/* If the length is not specified, use a sensible default */

	if (length <= 0) {
		length = 16;
	}

	/* Allocate the new ArrayList and fill in the fields.  There are
	 * initially no entries. */

	new_arraylist = (ArrayList *) malloc(sizeof(ArrayList));

	if (new_arraylist == NULL) {
		return NULL;
	}

	new_arraylist->_alloced = length;
	new_arraylist->length = 0;

	/* Allocate the data array */

	new_arraylist->data = malloc(length * sizeof(ArrayListValue));

	if (new_arraylist->data == NULL) {
		free(new_arraylist);
		return NULL;
	}

	return new_arraylist;
}

void arraylist_free(ArrayList *arraylist)
{
	/* Do not free if a NULL pointer is passed */

	if (arraylist != NULL) {
		free(arraylist->data);
		free(arraylist);
	}
}
void test_arraylist_new_free(void)
{
	ArrayList *arraylist;

	/* Use a default size when given zero */

	arraylist = arraylist_new(0);
	assert(arraylist != NULL);
	arraylist_free(arraylist);

	/* Normal allocated */

	arraylist = arraylist_new(10);
	assert(arraylist != NULL);
	arraylist_free(arraylist);

	/* Freeing a null arraylist works */

	arraylist_free(NULL);

	/* Test low memory scenarios (failed malloc) */

	// alloc_test_set_limit(0);
	// arraylist = arraylist_new(0);
	// assert(arraylist == NULL);

	// alloc_test_set_limit(1);
	// arraylist = arraylist_new(100);
	// assert(arraylist == NULL);
}

int main(){
    test_arraylist_new_free();
    return 0;
}