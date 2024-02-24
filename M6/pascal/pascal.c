/**
 * @file pascal.c
 *
 * @brief Implementation of Pascal's triangle
 *
 * @author Christophe Garion
 *
 */

#include <stdio.h>

#include "pascal.h"

void create_pascal_triangle(int size, int array[size][size]) {
    if (size == 0) {
        return;
    }

    array[0][0] = 1;

    for (int i = 1; i < size; i++) {
        array[i][0] = 1;
        array[i][i] = 1;

        for (int j = 1; j < i; j++) {
            array[i][j] = array[i - 1][j - 1] + array[i - 1][j];
        }
    }
}

void print_pascal_triangle(int size, int array[size][size]) {
    for (int i = 0; i < size; i++) {
        for (int j = 0; j <= i; j++) {
            printf("%3d ", array[i][j]);
        }

        printf("\n");
    }
}
