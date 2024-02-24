#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int *p_array = calloc(5, sizeof(int));

    // p_array[1] = 1
    *(p_array + 1) = 1;

    // p_array[0] = 2
    *(p_array - 1) = 2;

    // p_array[3] = 100000
    *(p_array + 100000) = 3;

    return 0;
}
