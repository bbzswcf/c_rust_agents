#ifndef ALGORITHM_HASH_POINTER_H
#define ALGORITHM_HASH_POINTER_H
#ifdef __cplusplus
extern "C" {
#endif
unsigned int pointer_hash(void *location);
#ifdef __cplusplus
}
#endif
#endif 
#include <limits.h>
#include "hash-pointer.h"
unsigned int pointer_hash(void *location)
{
	return (unsigned int) (unsigned long) location;
}
