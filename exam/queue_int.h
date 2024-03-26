/**
 * @file queue_int.h
 *
 * @author C. Garion
 */

#ifndef QUEUE_INT_H
#define QUEUE_INT_H

#include <stdbool.h>

/**
 * @brief Type `struct cell_int` is aliased to `cell_int` for
 *        readability
 */
typedef struct cell_int cell_int;

/**
 * @brief The structure representing cells in the list
 */
struct cell_int {
    /** The value contained in the cell */
    int       value;
    /** A pointer to the next cell */
    cell_int *p_next;
};

/**
 * @brief Type `struct queue_int` is aliased to `queue_int` for
 *        readability
 */
typedef struct queue_int queue_int;

/**
 * @brief The structure representing a queue
 */
struct queue_int {
    /** A pointer to the first cell of the queue */
    cell_int *p_first;
    /** A pointer to the last cell of the queue */
    cell_int *p_last;
    /** The size of the queue, i.e. the number of cells in the queue */
    int       size;
};

/**
 * Create a new empty queue.
 *
 * @return an empty queue
 */
queue_int nil();

/**
 * Get the size of the queue.
 *
 * @param queue the queue
 *
 * @return the number of cells in the queue
 */
int size(queue_int queue);

/**
 * Is the queue empty?
 *
 * @param queue the queue to be tested
 *
 * @return a boolean that is `true` if the queue is empty
 */
bool is_empty(queue_int queue);

/**
 * Add a new value in the queue
 *
 * @param p_queue a pointer to the queue in which the new
 *                value has to be inserted
 * @param value   the value to be inserted
 */
 void enqueue(queue_int *p_queue, int value);

/**
 * Remove the oldest cell in the queue and return its value.
 *
 * @param p_queue a pointer to the queue from which the oldest cell
 *                has to be deleted
 *
 * @return the value in the oldest cell of the queue
 */
int dequeue(queue_int *p_queue);

/**
 * Append two queues. The queues passed as parameters may be modified!
 *
 * @param queue_1 the first queue to be appended
 * @param queue_2 the second queue to be appended
 *
 * @return a queue with all values of `queue_1` and `queue_2` such
 *         that its oldest value is the oldest value of `queue_1`
 *         and its newest value if the newest value of `queue_2`
 */
queue_int append(queue_int queue_1, queue_int queue_2);

/**
 * Copy a queue.
 *
 * @param queue the queue to be copied
 *
 * @return a queue with all the values of `queue` in the same order.
 *         The cells of the returned queue must be different from
 *         the cells of `queue` (i.e. not at the same memory region)
 */
queue_int copy(queue_int queue);

#endif
