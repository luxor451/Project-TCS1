/**
 * @file pascal.c
 *
 * @brief Implementation of Pascal's triangle
 *
 * @author Christophe Garion
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "pascal.h"

void create_pascal_triangle(int size, int** array) {
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

void print_pascal_triangle(int size, int** array) {
    for (int i = 0; i < size; i++) {
        for (int j = 0; j <= i; j++) {
            printf("%3d ", array[i][j]);
        }

        printf("\n");
    }
}


int* create_single_row(int size){
    int* row = malloc(sizeof(int) * size);
    assert(row != NULL);
    return row;
}

void print_array(int size, int* arr){
    for (int i = 0; i < size; i++)
    {
        printf("%d ", arr[i]);
    }
    
}

int** create_pointers_array(int size){
    int** arr = (int**)malloc(sizeof(int*)*size);
    assert(arr != NULL);
    return arr;
}


int** create_pascal_triangle_iliffe(int size) {
    int** triangle = create_pointers_array(size);
    for (int i = 0; i < size; i++)
    {
        triangle[i] = create_single_row(i+1);
    }
    create_pascal_triangle(size, triangle);
    return triangle;
}

void free_triangle(int size, int** arr){
    for (int i = 0; i < size; i++)
    {
        free(arr[i]);
    }
    free(arr);
}