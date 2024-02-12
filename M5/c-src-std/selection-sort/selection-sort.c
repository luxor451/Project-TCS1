#include <stdio.h>


void print_array(int array[], int size) {
    printf("[ ");

    for (int i = 0; i < size; i++) {
        printf("%d%s", array[i], i == size - 1 ? " " : ", ");
    }

    printf("]");
}

void swap(int array[], int i, int j) {}

int index_of_minimum(int array[], int size, int start_from) {
    return start_from;
}

void selection_sort(int array[], int size) {
    for (int i = 0; i < size - 1; i++) {
        swap(array, i, index_of_minimum(array, size, i));
    }
}
