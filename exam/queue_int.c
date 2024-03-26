#include "queue_int.h"
#include <stdlib.h>
#include <assert.h>

queue_int nil() {
    queue_int* res = malloc(sizeof(queue_int));
    res->p_first = NULL;
    res->p_last = NULL;
    res->size = 0;
    return *res;
}

int size(queue_int queue) {
    return queue.size;
}

bool is_empty(queue_int queue) {
    return queue.p_last == 0;
}


void enqueue(queue_int *p_queue, int value) {
    cell_int *new_node = malloc(sizeof(new_node));
    new_node->value = value;
    if (is_empty(*p_queue))
    {
        p_queue->p_first = new_node;
        p_queue->p_last = new_node;
        p_queue->size = 1;
    } else {
        p_queue->p_last->p_next = new_node;
        p_queue->p_last = new_node;
    }
    
}

int dequeue(queue_int *p_queue){
    assert(!is_empty(*p_queue));
    cell_int* first_node = p_queue->p_first;
    cell_int* next_node = first_node->p_next;
    int res = first_node->value;
    p_queue->p_first = next_node;
    p_queue->size -= 1;
    free(first_node);
    return res;
}
