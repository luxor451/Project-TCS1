/**
 * @file pascal.h
 *
 * @brief Sorting C arrays using selection sort algorithm
 *
 * @author Christine Tasson
 *
 */

#ifndef SELECTION-SORT_H_
#define SELECTION-SORT_H_


void print_array(int array[], int size);


void swap(int array[], int i, int j);


int index_of_minimum(int array[], int size, int start_from);


void selection_sort(int array[], int size);




#endif // SELECTION-SORT_H_
