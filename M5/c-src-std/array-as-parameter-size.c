#include <stdio.h>

void print_array_as_parameter(int array[]) {
    printf("size of array as parameter: %ld\n", sizeof(array));
}

int main(void) {
    int array[] = { 1, 2, 3, 4, 5, 6 };

    printf("size of array: %ld\n", sizeof(array));
    print_array_as_parameter(array);

    return 0;
}
