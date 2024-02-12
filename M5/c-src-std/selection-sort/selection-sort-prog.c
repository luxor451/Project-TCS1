/**
 * @file selection-sort-prog.c
 *
 * @brief Test selection-sort implementation
 *
 * @author Christophe Garion
 *
 */



#include "selection-sort.h"



int main(void) {
    int size       = 6;
    int my_array[] = { -1, 0, 14, 7, -8, 10 };

    printf("Array before sorting: ");
    print_array(my_array, size);
    printf("\n");

    selection_sort(my_array, size);

    printf("Array after sorting: ");
    print_array(my_array, size);
    printf("\n");

    return 0;
}
