#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include "linked_list_int.h"

linked_list_int nil(){
    return NULL;
}

linked_list_int new_cell(int elem){
    linked_list_int res = NULL;
    res = malloc(sizeof(cell_int));
    assert(res != NULL);
    res->e = elem;
    res->next = NULL;
    return res;
}

void link_cell(linked_list_int cell1, linked_list_int cell2){
    assert(cell1->next == NULL);
    cell1->next= cell2;
}


void cons(int elem, linked_list_int* l){
    linked_list_int maillon = new_cell(elem);
    link_cells(maillon, *l);
    *l = maillon;
}

int size(linked_list_int l){
    int res= 0;
    for (cell_int* p = l; p != NULL; p = p->next)
    {
        res++;
    }
    return res;   
}

bool is_bool(linked_list_int l){
    return l == NULL;
}

int get_elem(linked_list_int l){
    int res = l->e;
    for (cell_int* p = l; p->next != NULL; p = p->next)
    {
        res = p->e;
    }
    return res;   
}

void insert_elem(int e, linked_list_int l){

}