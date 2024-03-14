/**
 * @file ring_list.h
 *
 * @author C. Garion
 */

#ifndef RING_LIST_H
#define RING_LIST_H

#include <stdbool.h>

/**
 * @brief Type `struct cell_int` is aliased to `cell_int` for
 *        readability
 */
typedef struct cell_int cell_int;

/**
 * @brief A ring list is a pointer to a cell of the list
 */
typedef struct cell_int * ring_list;

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
 * Create a new empty ring list.
 *
 * @return an empty ring list
 */
ring_list create_empty_ring();

/**
 * Is the list empty?
 *
 * @param list the list to be tested
 *
 * @return a boolean that is `true` if the list is empty
 */
bool is_empty(ring_list list);

/**
 * Insert a new cell in the list.
 *
 * @param list the list in which the cell has to be inserted
 *
 * @param value the value contained in the new cell
 *
 * @return the list passed as parameter, but with a new cell containing
 *         `value` inserted after the first cell if it exists. If `list`
 *         is empty, the returned list contains a single cell.
 */
ring_list insert(ring_list list, int value);

/**
 * Rotate a ring list, i.e. put the pointer on the first cell on another cell.
 *
 * @param list the list to be rotated
 *
 * @param n the number of atomic shifts to be performed on the list
 *
 * @return a list identical to the list passed as parameter, but with the
 *         first cell moved accordingly to `n`
 */
ring_list rotate(ring_list list, int n);

/**
 * Get the size of the ring list, i.e. the number of cells inside it.
 *
 * @param list the list
 *
 * @return the number of cells in the list
 */
int size(ring_list list);

/**
 * Append `list_1` to `list_2`.
 *
 * @param list_1 the first list
 *
 * @param list_2 the list to be appended
 *
 * @return a ring list containing `list_1` concatanated to `list_2`.
 *         Notice that both list will be modified.
 */
ring_list append(ring_list list_1, ring_list list_2);

#endif
