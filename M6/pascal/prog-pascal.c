/**
 * @file prog-pascal.c
 *
 * @brief Test Pascal's triangle implementation
 *
 * @author Christophe Garion
 *
 */

#include <stdio.h>

#include "pascal.h"

int main(void) {
    int size = 4;
    int array[size][size];

    create_pascal_triangle(size, array);
    print_pascal_triangle(size, array);

    return 0;
}
