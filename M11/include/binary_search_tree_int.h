/**
 * @file binary_search_tree_int.h
 *
 * @brief Simple definition of BST containing `int` values
 *
 * @author Christophe Garion
 *
 * This is a simple definition of a binary search tree containing `int`
 * values implemented using nodes linked by pointers. Warning: the cells
 * pointers should not be shared as deallocation of a BST
 * deallocates the memory space reserved for the cells!
 *
 * The API of the BST is defined as follows:
 *
 * - a function to create an empty BST
 * - a function to check if the BST is empty
 * - a function to get the value at the root of the BST
 * - a function to get the left child of the BST
 * - a function to get the right child of the BST
 * - a function to get the size of the BST
 * - a function to get the height of the BST
 * - a function to retrieve a value in the BST
 * - a function to insert a value in the BST
 * - a function to delete a value in the BST
 * - a function to print the nodes values in-order
 * - a function to deallocate the BST
 * - a function to print the BST
 */

#ifndef BINARY_SEARCH_TREE_INT_H
#define BINARY_SEARCH_TREE_INT_H

#include <stdbool.h>

/**
 * @brief An alias to the structure representing the nodes of
 *        the tree.
 */
typedef struct bst_node_int bst_node_int;

/**
 * @brief A binary tree is just a pointer to the root node
 *        of the tree.
 */
typedef bst_node_int *bst_int;

/**
 * @brief The structure representing the nodes of the
 *        binary tree.
 */
struct bst_node_int {
    /** The left subtree of the node */
    bst_int left;
    /** The value/key in the node */
    int     value;
    /** The right subtree of the node */
    bst_int right;
};

/**
 * @brief Create a nil binary search tree.
 *
 * @return an empty binary search tree
 */
bst_int nil();

/**
 * @brief Is the binary search tree empty?
 *
 * @param tree  a the tree you want to check the emptiness
 *
 * @return `true` if `tree` is empty, `false` else
 */
bool is_empty(bst_int tree);

/**
 * @brief The value in the root of the binary search tree.
 *
 * @param tree  the tree
 *
 * @pre `tree` is not empty.
 *
 * @return the value in the root of the tree
 */
int value(bst_int tree);

/**
 * @brief The left subtree of the tree.
 *
 * @param tree the tree
 *
 * @pre `tree` is not empty.
 *
 * @return the left subtree of `tree`
 */
bst_int left_child(bst_int tree);

/**
 * @brief The right subtree of the tree.
 *
 * @param tree the tree
 *
 * @pre `tree` is not empty.
 *
 * @return the right subtree of `tree`
 */
bst_int right_child(bst_int tree);

/**
 * @brief The number of nodes in the binary search tree.
 *
 * @param tree the tree
 *
 * @return the number of nodes in the tree
 */
int size(bst_int tree);

/**
 * @brief The height of the binary search tree as defined in the lecture.
 * In particular, we consider that the height of an empty tree is `-1`.
 *
 * @param tree the tree
 *
 * @return the height of the tree
 */
int height(bst_int tree);

/**
 * @brief Retrieve a subtree in the tree given the value of
 *        its root.
 *
 * @param tree   a pointer   to the tree
 * @param value_to_retrieve  the value of the root of the subtree to search
 *
 * @return an empty tree if the value is not in
 *         the binary search tree, the subtree
 *         whose root contains the value otherwise
 */
bst_int retrieve(bst_int tree, int value_to_retrieve);

/**
 * @brief Insert a value in the tree.
 *
 * @param tree             the tree in which the value is to be inserted
 * @param value_to_insert  the value to be inserted
 *
 * @return a tree in which there is a node containing `value`.
 *         This node is correctly placed with respect to the
 *         binary search tree property. If the node was already in
 *         the tree, no new node is inserted and `tree` is returned.
 */
bst_int insert(bst_int tree, int value_to_insert);

#ifndef STD
/**
 * @brief Insert a value in the tree (pure "pointer" version).
 *
 * @param tree             the tree in which the value is to be inserted
 * @param value_to_insert  the value to be inserted
 *
 * @return a tree in which there is a node containing `value`.
 *         This node is correctly placed with respect to the
 *         binary search tree property. If the node was already in
 *         the tree, no new node is inserted and `tree` is returned.
 */
bst_int insert_using_pointers(bst_int tree, int value_to_insert);

#endif
/**
 * @brief Delete a value in the tree.
 *
 * @param tree             the tree in which the value is to be deleted
 * @param value_to_delete  the value to be deleted
 *
 * @return a tree such that if there was a node containing
 *       `value` in `tree`, then this node is deleted.
 *       This deletion is such that the binary
 *       search property is verified. If the node was
 *       not in the tree, `tree` is returned.
 */
bst_int delete(bst_int tree, int value_to_delete);

#ifndef STD
/**
 * @brief Delete a value in the tree (pure "pointer" version).
 *
 * @param tree             the tree in which the value is to be deleted
 * @param value_to_delete  the value to be deleted
 *
 * @return a tree such that if there was a node containing
 *       `value` in `tree`, then this node is deleted.
 *       This deletion is such that the binary
 *       search property is verified. If the node was
 *       not in the tree, `tree` is returned.
 */
bst_int delete_using_pointers(bst_int tree, int value_to_delete);

#endif
/**
 * @brief Traverse the tree depth-first and print the value
 *        in order (infix style).
 *
 * @param tree  the tree to be traversed
 *
 * @post After the call, the values in the nodes are printed
 *       in order.
 */
void in_order_dfs_infix(bst_int tree);

/**
 * @brief Deallocate a binary search tree.
 *
 * @param tree  the tree to be deallocated
 *
 * @post After the call, all memory regions used for the nodes
 *       are deallocated
 */
void deallocate_bst(bst_int tree);

/**
 * @brief Prints the tree.
 *
 * @param tree  the tree to be printed
 *
 * @post After the call, the tree is printed.
 */
void print_bst(bst_int tree);

#endif
