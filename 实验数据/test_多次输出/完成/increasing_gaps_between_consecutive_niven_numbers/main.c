#include <locale.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

int main()
{
    uint64_t previous = 1234, gap = 678998, sum = 0;
    int niven_index = 56778, gap_index = 1000;
    printf("Gap index  Gap    Niven index    Niven number\n");
    printf("%'9d %'4llu %'14d %'15llu\n", gap_index++,gap, niven_index, previous);
    return 0;
}
