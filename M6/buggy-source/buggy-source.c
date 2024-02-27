#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int *p_array = calloc(5, sizeof(int));

    // p_array[1] = 1
    *(p_array + 1) = 1;

    // p_array[0] = 2
    *(p_array) = 2;

    // p_array[3] = 100000
    *(p_array + 3) = 100000;

    free(p_array);
    return 0;
}
