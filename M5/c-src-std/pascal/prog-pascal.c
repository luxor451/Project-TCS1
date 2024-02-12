/**
 * @file prog-pascal.c
 *
 * @brief Test Pascal's triangle implementation
 *
 * @author Christophe Garion
 *
 */

#include "pascal.h"

int main(void)
{
    int size = 10;
    int array[size][size];

    create_pascal_triangle(size, array);
    print_pascal_triangle_array(size, array);
}
