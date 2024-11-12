#ifndef ALGORITHM_HASH_STRING_H
#define ALGORITHM_HASH_STRING_H
#ifdef __cplusplus
extern "C" {
#endif
unsigned int string_hash(void *string);
unsigned int string_nocase_hash(void *string);
#ifdef __cplusplus
}
#endif
#endif 
#include <ctype.h>
#include "hash-string.h"
unsigned int string_hash(void *string)
{
	unsigned int result = 5381;
	unsigned char *p;
	p = (unsigned char *) string;
	while (*p != '\0') {
		result = (result << 5) + result + *p;
		++p;
	}
	return result;
}
unsigned int string_nocase_hash(void *string)
{
	unsigned int result = 5381;
	unsigned char *p;
	p = (unsigned char *) string;
	while (*p != '\0') {
		result = (result << 5) + result + (unsigned int) tolower(*p);
		++p;
	}
	return result;
}
