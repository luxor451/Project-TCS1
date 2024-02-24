#include <stdio.h>
#include <stdlib.h>

int *create_array_of_two_ints() {
    int *p = malloc(2);
    if (p == NULL) {
        printf("error: cannot allocate memory!\n");
    }

    return p;
}

int main(void) {
    // use function to create an array of 2 ints
    int *p_two_array = create_array_of_two_ints();

    p_two_array[0] = 1;
    p_two_array[1] = 2;

    // create a dynamic array of 5 ints
    int *p_five_array = calloc(5, sizeof(int));

    if (p_five_array == NULL) {
        printf("error: cannot allocate memory!\n");
    }

    for (int i = 0; i < 5; i++) {
        printf("%d\n", p_five_array[i]);
    }

    // keep only the first integer in the array
    free(p_five_array + 1);

    *p_five_array = 2;

    printf("%d\n", *p_five_array);

    free(p_five_array);

    *p_five_array = 5;

    return 0;
}
