#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <stdbool.h>
#include "linked_list_int.h"



linked_list_int new_cell(int elem){
    linked_list_int res = NULL;
    res = malloc(sizeof(cell_int));
    assert(res != NULL);
    res->e = elem;
    res->next = NULL;
    return res;
}

linked_list_int get_out_list(linked_list_int l, int pos){
    linked_list_int current = l;
    if (pos == 0)
    {
        NULL;
    }
    else 
    {
        for (int i = 0; i < pos-1; i++)
        {
            current = current->next;
        }
    }
    return current;
}

linked_list_int nil(){
    return NULL;
}

void link_cells(linked_list_int cell1, linked_list_int cell2){
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

bool is_empty(linked_list_int l){
    return l == NULL;
}

int get_elem(linked_list_int l, int pos){
    assert(pos <= size(l) - 1);
    linked_list_int current = l;
    for (int i = 0; i < pos; i++)
    {
        current = current->next;
    }
    return current->e;
}



void insert_elem(linked_list_int l, int pos, int e){
    assert(pos <= size(l));
    linked_list_int new_maillion = new_cell(e);
    linked_list_int current = get_out_list(l, pos);
    link_cells(new_maillion, current->next);
    link_cells(current, new_maillion);
}

void remove_elem(linked_list_int l, int pos){
    assert(pos <= size(l) - 1);
    linked_list_int current = get_out_list(l, pos);
    current = current->next;
}

void dealocate_list(linked_list_int l){
    linked_list_int current;
    while (l != NULL) {
        current = l;
        l = l->next;
        free(current);
    }
}