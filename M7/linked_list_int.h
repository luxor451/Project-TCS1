#ifndef LINKED_LIST_INT_H
#define LINKED_LIST_INT_H
#include <stdbool.h>

typedef struct cell_int  cell_int;
typedef cell_int* linked_list_int;

struct cell_int {
    int e;
    cell_int* next;
};

linked_list_int nil();

void cons(int, linked_list_int*);

int size(linked_list_int);

bool is_empty(linked_list_int);

void get_element(linked_list_int, int);

void insert_element(int, linked_list_int);

void remove_element(int, linked_list_int);

void deallocate_list(linked_list_int);

void print_list(linked_list_int);

#endif
