/**
 * @file binary_search_tree_int_tests.c
 *
 * @brief Unit tests for binary search trees containing `int` values.
 *
 * @author Christophe Garion
 */

#define _GNU_SOURCE
#include "unity_fixture.h"
// do not use Unity malloc and free versions!
#undef malloc
#undef free

#include <assert.h>
#include <error.h>
#include <getopt.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include"binary_search_tree_int.h"

static bst_int test_bst;

// utility function to create a BST like a linked list
// (going right)
bst_int create_bst_right(int nb) {
    bst_int       tree           = NULL;
    bst_node_int *p_current_node = NULL;

    for (int i = 0; i < nb; i++) {
        if (i == 0) {
            tree = malloc(sizeof(bst_node_int));

            tree->value = i;
            tree->left  = NULL;
            tree->right = NULL;

            p_current_node = tree;

            continue;
        }

        p_current_node->right = malloc(sizeof(bst_node_int));

        p_current_node->right->value = i;
        p_current_node->right->left  = NULL;
        p_current_node->right->right = NULL;

        p_current_node = p_current_node->right;
    }

    return tree;
}

// utility function to create a BST like a linked list
// (going left)
bst_int create_bst_left(int nb) {
    bst_int       tree           = NULL;
    bst_node_int *p_current_node = NULL;

    for (int i = 0; i < nb; i++) {
        if (i == 0) {
            tree = malloc(sizeof(bst_node_int));

            tree->value = nb;
            tree->left  = NULL;
            tree->right = NULL;

            p_current_node = tree;

            continue;
        }

        p_current_node->left = malloc(sizeof(bst_node_int));

        p_current_node->left->value = nb - i;
        p_current_node->left->left  = NULL;
        p_current_node->left->right = NULL;

        p_current_node = p_current_node->left;
    }

    return tree;
}

static int values[]   = { 3, 1, 5, 0, 2, 4, 6, 7};
static int heights[]  = { -1, 0, 1, 1, 2, 2, 2, 2, 3};
static char *paths[]  = { "",
                         "0",
                         "1",
                         "00",
                         "01",
                         "10",
                         "11",
                         "111"
                        };

// utility function to create a BST from the previous tabs
bst_int create_bst(int nb) {
    bst_int tree = NULL;

    for (int i = 0; i < nb; i++) {
        if (i == 0) {
            tree = malloc(sizeof(bst_node_int));

            tree->value = values[i];
            tree->left  = NULL;
            tree->right = NULL;

            continue;
        }

        bst_node_int *p_current_node = tree;

        char *str_path   = paths[i];
        int  path_length = strlen(str_path);

        for (int j = 0; j < path_length - 1; j++) {
            if (str_path[j] == '0') {
                p_current_node = p_current_node->left;
            } else {
                p_current_node = p_current_node->right;
            }
        }

        if (str_path[path_length - 1] == '0') {
            p_current_node->left = malloc(sizeof(bst_node_int));

            p_current_node->left->value = values[i];
            p_current_node->left->left  = NULL;
            p_current_node->left->right = NULL;
        } else {
            p_current_node->right = malloc(sizeof(bst_node_int));

            p_current_node->right->value = values[i];
            p_current_node->right->left  = NULL;
            p_current_node->right->right = NULL;
        }

    }

    return tree;
}

// utility function to check the BST property using in-order DFT
int in_order_dpt_tab(bst_int tree, int tab[], int index) {
    if (tree == NULL) {
        return index;
    }

    int new_index = index;

    new_index      = in_order_dpt_tab(tree->left, tab, new_index);
    tab[new_index] = tree->value;
    return in_order_dpt_tab(tree->right, tab, new_index + 1);
}

void check_bst_property(bst_int tree, int size) {
    if (tree == NULL) {
        return;
    }

    int tab[size];

    for (int i = 0; i < size; i++) {
        tab[i] = -1;
    }

    in_order_dpt_tab(tree, tab, 0);

    for (int i = 0; i < size - 1; i++) {
        char *msg;

        asprintf(&msg,
                 "I have found %d before %d in in-order depth-first traversal!",
                 tab[i], tab[i + 1]);

        TEST_ASSERT_TRUE_MESSAGE(tab[i] < tab[i + 1], msg);

        free(msg);
    }
}

// utility function to find BST size
int my_size(bst_int tree) {
    if (tree == NULL) {
        return 0;
    }

    return 1 + my_size(tree->left) +
        my_size(tree->right);
}

/* Tests for nil */

TEST_GROUP(nil);

TEST_SETUP(nil) {
    test_bst = nil();
};

TEST_TEAR_DOWN(nil) {
    deallocate_bst(test_bst);
    test_bst = NULL;
};

TEST(nil, linked_bst_int_nil) {
    TEST_ASSERT_NULL_MESSAGE(test_bst,
        "the BST should be NULL!");
}

TEST_GROUP_RUNNER(nil) {
    printf("\n--------------------\n");
    printf("Testing nil function\n\n");

    RUN_TEST_CASE(nil, linked_bst_int_nil);
};

/* Tests for is_empty */

TEST_GROUP(is_empty);

TEST_SETUP(is_empty) {

};

TEST_TEAR_DOWN(is_empty) {
    deallocate_bst(test_bst);
    test_bst = NULL;
};

// A macro to test is_empty on BST
#define TEST_BST_INT_IS_EMPTY(THE_SIZE)                                 \
    TEST(is_empty, bst_int_is_empty_##THE_SIZE) {                       \
        test_bst = create_bst(THE_SIZE);                                \
                                                                        \
        printf("testing is_empty with BST of size %d\n",                \
               THE_SIZE);                                               \
                                                                        \
        if (THE_SIZE == 0) {                                            \
            TEST_ASSERT_TRUE_MESSAGE(is_empty(test_bst),                \
                                     "The BST should be empty!");       \
                                                                        \
        } else {                                                        \
            TEST_ASSERT_FALSE_MESSAGE(is_empty(test_bst),               \
                                      "The BST should not be empty!");  \
        }                                                               \
    };

TEST_BST_INT_IS_EMPTY(0);
TEST_BST_INT_IS_EMPTY(1);
TEST_BST_INT_IS_EMPTY(2);
TEST_BST_INT_IS_EMPTY(3);
TEST_BST_INT_IS_EMPTY(4);
TEST_BST_INT_IS_EMPTY(5);
TEST_BST_INT_IS_EMPTY(6);
TEST_BST_INT_IS_EMPTY(7);

TEST_GROUP_RUNNER(is_empty) {
    printf("\n-------------------------\n");
    printf("Testing is_empty function\n\n");

    RUN_TEST_CASE(is_empty, bst_int_is_empty_0);
    RUN_TEST_CASE(is_empty, bst_int_is_empty_1);
    RUN_TEST_CASE(is_empty, bst_int_is_empty_2);
    RUN_TEST_CASE(is_empty, bst_int_is_empty_3);
    RUN_TEST_CASE(is_empty, bst_int_is_empty_4);
    RUN_TEST_CASE(is_empty, bst_int_is_empty_5);
    RUN_TEST_CASE(is_empty, bst_int_is_empty_6);
    RUN_TEST_CASE(is_empty, bst_int_is_empty_7);
};

/* Tests for size */

TEST_GROUP(size);

TEST_SETUP(size) {

};

TEST_TEAR_DOWN(size) {
    deallocate_bst(test_bst);
    test_bst = NULL;
};

// A macro to test size on BST
#define TEST_BST_INT_SIZE(THE_SIZE)                                     \
    TEST(size, bst_int_size_##THE_SIZE) {                               \
        test_bst = create_bst(THE_SIZE);                                \
                                                                        \
        printf("testing size with BST of size %d\n",                    \
               THE_SIZE);                                               \
                                                                        \
        TEST_ASSERT_EQUAL_INT_MESSAGE(THE_SIZE, size(test_bst),         \
                                      "The size should be THE_SIZE");   \
                                                                        \
        check_bst_property(test_bst, THE_SIZE);                         \
    };

TEST_BST_INT_SIZE(0);
TEST_BST_INT_SIZE(1);
TEST_BST_INT_SIZE(2);
TEST_BST_INT_SIZE(3);
TEST_BST_INT_SIZE(4);
TEST_BST_INT_SIZE(5);
TEST_BST_INT_SIZE(6);
TEST_BST_INT_SIZE(7);

TEST_GROUP_RUNNER(size) {
    printf("\n---------------------\n");
    printf("Testing size function\n\n");

    RUN_TEST_CASE(size, bst_int_size_0);
    RUN_TEST_CASE(size, bst_int_size_1);
    RUN_TEST_CASE(size, bst_int_size_2);
    RUN_TEST_CASE(size, bst_int_size_3);
    RUN_TEST_CASE(size, bst_int_size_4);
    RUN_TEST_CASE(size, bst_int_size_5);
    RUN_TEST_CASE(size, bst_int_size_6);
    RUN_TEST_CASE(size, bst_int_size_7);
};

/* Tests for value */

TEST_GROUP(value);

TEST_SETUP(value) {

};

TEST_TEAR_DOWN(value) {
    deallocate_bst(test_bst);
    test_bst = NULL;
};

// A macro to test value on BST build like a linked list (left)
#define TEST_BST_INT_VALUE_LEFT(THE_SIZE)                               \
    TEST(value, bst_int_value_left_##THE_SIZE) {                        \
        test_bst = create_bst_left(THE_SIZE);                           \
                                                                        \
        printf("testing value with BST of size %d like a (left) linked list (value at root %d)\n", \
               THE_SIZE, THE_SIZE);                                     \
                                                                        \
        char *msg;                                                      \
        asprintf(&msg,                                                  \
                 "The value at the root should be %d",                  \
                 THE_SIZE);                                             \
                                                                        \
        TEST_ASSERT_EQUAL_INT_MESSAGE(THE_SIZE, value(test_bst),        \
                                      msg);                             \
                                                                        \
        free(msg);                                                      \
                                                                        \
        check_bst_property(test_bst, THE_SIZE);                         \
    };

// A macro to test value on BST build like a linked list (right)
#define TEST_BST_INT_VALUE_RIGHT(THE_SIZE)                              \
    TEST(value, bst_int_value_right_##THE_SIZE) {                       \
        test_bst = create_bst_right(THE_SIZE);                          \
                                                                        \
        printf("testing value with BST of size %d like a (right) linked list (value at root %d)\n", \
               THE_SIZE, THE_SIZE);                                     \
                                                                        \
        char *msg;                                                      \
        asprintf(&msg,                                                  \
                 "The value of the tree should be %d",                  \
                 0);                                                    \
                                                                        \
        TEST_ASSERT_EQUAL_INT_MESSAGE(0, value(test_bst),               \
                                      msg);                             \
                                                                        \
        free(msg);                                                      \
                                                                        \
        check_bst_property(test_bst, THE_SIZE);                         \
    };

TEST_BST_INT_VALUE_LEFT(1);
TEST_BST_INT_VALUE_LEFT(2);
TEST_BST_INT_VALUE_LEFT(3);
TEST_BST_INT_VALUE_LEFT(4);
TEST_BST_INT_VALUE_LEFT(5);
TEST_BST_INT_VALUE_LEFT(6);
TEST_BST_INT_VALUE_LEFT(7);
TEST_BST_INT_VALUE_RIGHT(1);
TEST_BST_INT_VALUE_RIGHT(2);
TEST_BST_INT_VALUE_RIGHT(3);
TEST_BST_INT_VALUE_RIGHT(4);
TEST_BST_INT_VALUE_RIGHT(5);
TEST_BST_INT_VALUE_RIGHT(6);
TEST_BST_INT_VALUE_RIGHT(7);

TEST_GROUP_RUNNER(value) {
    printf("\n----------------------\n");
    printf("Testing value function\n\n");

    RUN_TEST_CASE(value, bst_int_value_left_1);
    RUN_TEST_CASE(value, bst_int_value_left_2);
    RUN_TEST_CASE(value, bst_int_value_left_3);
    RUN_TEST_CASE(value, bst_int_value_left_4);
    RUN_TEST_CASE(value, bst_int_value_left_5);
    RUN_TEST_CASE(value, bst_int_value_left_6);
    RUN_TEST_CASE(value, bst_int_value_left_7);

    RUN_TEST_CASE(value, bst_int_value_right_1);
    RUN_TEST_CASE(value, bst_int_value_right_2);
    RUN_TEST_CASE(value, bst_int_value_right_3);
    RUN_TEST_CASE(value, bst_int_value_right_4);
    RUN_TEST_CASE(value, bst_int_value_right_5);
    RUN_TEST_CASE(value, bst_int_value_right_6);
    RUN_TEST_CASE(value, bst_int_value_right_7);
};

/* Tests for height */

TEST_GROUP(height);

TEST_SETUP(height) {

};

TEST_TEAR_DOWN(height) {
    deallocate_bst(test_bst);
    test_bst = NULL;
};

// A macro to test height on BST build like a linked list (left)
#define TEST_BST_INT_HEIGHT_LEFT(THE_SIZE)                              \
    TEST(height, bst_int_height_left_##THE_SIZE) {                      \
        test_bst = create_bst_left(THE_SIZE);                           \
                                                                        \
        printf("testing height with BST of size %d like a (left) linked list (height %d)\n", \
               THE_SIZE, THE_SIZE);                                     \
                                                                        \
        char *msg;                                                      \
        asprintf(&msg,                                                  \
                 "The height of the tree should be %d",                 \
                 THE_SIZE);                                             \
                                                                        \
        TEST_ASSERT_EQUAL_INT_MESSAGE(THE_SIZE - 1, height(test_bst),   \
                                      msg);                             \
                                                                        \
        free(msg);                                                      \
                                                                        \
        check_bst_property(test_bst, THE_SIZE);                         \
    };

// A macro to test height on BST build like a linked list (right)
#define TEST_BST_INT_HEIGHT_RIGHT(THE_SIZE)                             \
    TEST(height, bst_int_height_right_##THE_SIZE) {                     \
        test_bst = create_bst_right(THE_SIZE);                          \
                                                                        \
        printf("testing height with BST of size %d like a (right) linked list (height %d)\n", \
               THE_SIZE, THE_SIZE);                                     \
                                                                        \
        char *msg;                                                      \
        asprintf(&msg,                                                  \
                 "The height of the tree should be %d",                 \
                 THE_SIZE - 1);                                         \
                                                                        \
        TEST_ASSERT_EQUAL_INT_MESSAGE(THE_SIZE - 1, height(test_bst),   \
                                      msg);                             \
                                                                        \
        free(msg);                                                      \
                                                                        \
        check_bst_property(test_bst, THE_SIZE);                         \
    };

// A macro to test height on the example BST
#define TEST_BST_INT_HEIGHT(THE_SIZE)                                   \
    TEST(height, bst_int_height_##THE_SIZE) {                           \
        test_bst = create_bst(THE_SIZE);                                \
                                                                        \
        printf("testing height with example BST of size %d\n",          \
               THE_SIZE);                                               \
                                                                        \
        char *msg;                                                      \
        asprintf(&msg,                                                  \
                 "The height of the tree should be %d",                 \
                 heights[THE_SIZE]);                                    \
                                                                        \
        TEST_ASSERT_EQUAL_INT_MESSAGE(heights[THE_SIZE], height(test_bst), \
                                      msg);                             \
                                                                        \
        free(msg);                                                      \
                                                                        \
        check_bst_property(test_bst, THE_SIZE);                         \
    };

TEST_BST_INT_HEIGHT_LEFT(0);
TEST_BST_INT_HEIGHT_LEFT(1);
TEST_BST_INT_HEIGHT_LEFT(2);
TEST_BST_INT_HEIGHT_LEFT(3);
TEST_BST_INT_HEIGHT_LEFT(4);
TEST_BST_INT_HEIGHT_LEFT(5);
TEST_BST_INT_HEIGHT_LEFT(6);
TEST_BST_INT_HEIGHT_LEFT(7);
TEST_BST_INT_HEIGHT_RIGHT(0);
TEST_BST_INT_HEIGHT_RIGHT(1);
TEST_BST_INT_HEIGHT_RIGHT(2);
TEST_BST_INT_HEIGHT_RIGHT(3);
TEST_BST_INT_HEIGHT_RIGHT(4);
TEST_BST_INT_HEIGHT_RIGHT(5);
TEST_BST_INT_HEIGHT_RIGHT(6);
TEST_BST_INT_HEIGHT_RIGHT(7);
TEST_BST_INT_HEIGHT(0);
TEST_BST_INT_HEIGHT(1);
TEST_BST_INT_HEIGHT(2);
TEST_BST_INT_HEIGHT(3);
TEST_BST_INT_HEIGHT(4);
TEST_BST_INT_HEIGHT(5);
TEST_BST_INT_HEIGHT(6);
TEST_BST_INT_HEIGHT(7);

TEST_GROUP_RUNNER(height) {
    printf("\n-----------------------\n");
    printf("Testing height function\n\n");

    RUN_TEST_CASE(height, bst_int_height_left_0);
    RUN_TEST_CASE(height, bst_int_height_left_1);
    RUN_TEST_CASE(height, bst_int_height_left_2);
    RUN_TEST_CASE(height, bst_int_height_left_3);
    RUN_TEST_CASE(height, bst_int_height_left_4);
    RUN_TEST_CASE(height, bst_int_height_left_5);
    RUN_TEST_CASE(height, bst_int_height_left_6);
    RUN_TEST_CASE(height, bst_int_height_left_7);

    RUN_TEST_CASE(height, bst_int_height_right_0);
    RUN_TEST_CASE(height, bst_int_height_right_1);
    RUN_TEST_CASE(height, bst_int_height_right_2);
    RUN_TEST_CASE(height, bst_int_height_right_3);
    RUN_TEST_CASE(height, bst_int_height_right_4);
    RUN_TEST_CASE(height, bst_int_height_right_5);
    RUN_TEST_CASE(height, bst_int_height_right_6);
    RUN_TEST_CASE(height, bst_int_height_right_7);

    RUN_TEST_CASE(height, bst_int_height_0);
    RUN_TEST_CASE(height, bst_int_height_1);
    RUN_TEST_CASE(height, bst_int_height_2);
    RUN_TEST_CASE(height, bst_int_height_3);
    RUN_TEST_CASE(height, bst_int_height_4);
    RUN_TEST_CASE(height, bst_int_height_5);
    RUN_TEST_CASE(height, bst_int_height_6);
    RUN_TEST_CASE(height, bst_int_height_7);
};

/* Tests for left_child */

TEST_GROUP(left_child);

TEST_SETUP(left_child) {

};

TEST_TEAR_DOWN(left_child) {
    deallocate_bst(test_bst);
    test_bst = NULL;
};

// A macro to test left_child on BST build like a linked list (left)
#define TEST_BST_INT_LEFT_CHILD_LEFT(THE_DEPTH)                         \
    TEST(left_child, bst_int_left_child_left_##THE_DEPTH) {             \
        test_bst = create_bst_left(7);                                  \
                                                                        \
        printf("testing left_child with BST of size 7 like a (left) linked list (left_child applied %d times)\n", \
               THE_DEPTH);                                              \
                                                                        \
        bst_node_int *p_current_node = test_bst;                        \
                                                                        \
        for (int i = 0; i < THE_DEPTH; i++) {                           \
            p_current_node = left_child(p_current_node);                \
        }                                                               \
                                                                        \
        if (THE_DEPTH == 7) {                                           \
            TEST_ASSERT_NULL_MESSAGE(p_current_node,                    \
                                     "The subtree should be empty!");   \
        } else {                                                        \
            char *msg;                                                  \
            asprintf(&msg,                                              \
                     "The value of the left_child of the tree should be %d", \
                     7 - THE_DEPTH);                                    \
                                                                        \
            TEST_ASSERT_EQUAL_INT_MESSAGE(7 - THE_DEPTH,                \
                                          value(p_current_node),        \
                                          msg);                         \
                                                                        \
            free(msg);                                                  \
        }                                                               \
                                                                        \
        check_bst_property(test_bst, 7);                                \
    };

TEST_BST_INT_LEFT_CHILD_LEFT(0);
TEST_BST_INT_LEFT_CHILD_LEFT(1);
TEST_BST_INT_LEFT_CHILD_LEFT(2);
TEST_BST_INT_LEFT_CHILD_LEFT(3);
TEST_BST_INT_LEFT_CHILD_LEFT(4);
TEST_BST_INT_LEFT_CHILD_LEFT(5);
TEST_BST_INT_LEFT_CHILD_LEFT(6);
TEST_BST_INT_LEFT_CHILD_LEFT(7);

TEST_GROUP_RUNNER(left_child) {
    printf("\n---------------------------\n");
    printf("Testing left_child function\n\n");

    RUN_TEST_CASE(left_child, bst_int_left_child_left_0);
    RUN_TEST_CASE(left_child, bst_int_left_child_left_1);
    RUN_TEST_CASE(left_child, bst_int_left_child_left_2);
    RUN_TEST_CASE(left_child, bst_int_left_child_left_3);
    RUN_TEST_CASE(left_child, bst_int_left_child_left_4);
    RUN_TEST_CASE(left_child, bst_int_left_child_left_5);
    RUN_TEST_CASE(left_child, bst_int_left_child_left_6);
    RUN_TEST_CASE(left_child, bst_int_left_child_left_7);
};

/* Tests for right_child */

TEST_GROUP(right_child);

TEST_SETUP(right_child) {

};

TEST_TEAR_DOWN(right_child) {
    deallocate_bst(test_bst);
    test_bst = NULL;
};

// A macro to test right_child on BST build like a linked list (right)
#define TEST_BST_INT_RIGHT_CHILD_RIGHT(THE_DEPTH)                       \
    TEST(right_child, bst_int_right_child_right_##THE_DEPTH) {          \
        test_bst = create_bst_right(7);                                 \
                                                                        \
        printf("testing right_child with BST of size 7 like a (right) linked list (right_child applied %d times)\n", \
               THE_DEPTH);                                              \
                                                                        \
        bst_node_int *p_current_node = test_bst;                        \
                                                                        \
        for (int i = 0; i < THE_DEPTH; i++) {                           \
            p_current_node = right_child(p_current_node);               \
        }                                                               \
                                                                        \
        if (THE_DEPTH == 7) {                                           \
            TEST_ASSERT_NULL_MESSAGE(p_current_node,                    \
                                     "The subtree should be empty!");   \
        } else {                                                        \
            char *msg;                                                  \
            asprintf(&msg,                                              \
                     "The value of the right_child of the tree should be %d", \
                     THE_DEPTH);                                        \
                                                                        \
            TEST_ASSERT_EQUAL_INT_MESSAGE(THE_DEPTH,                    \
                                          value(p_current_node),        \
                                          msg);                         \
                                                                        \
            free(msg);                                                  \
        }                                                               \
                                                                        \
        check_bst_property(test_bst, 7);                                \
    };

TEST_BST_INT_RIGHT_CHILD_RIGHT(0);
TEST_BST_INT_RIGHT_CHILD_RIGHT(1);
TEST_BST_INT_RIGHT_CHILD_RIGHT(2);
TEST_BST_INT_RIGHT_CHILD_RIGHT(3);
TEST_BST_INT_RIGHT_CHILD_RIGHT(4);
TEST_BST_INT_RIGHT_CHILD_RIGHT(5);
TEST_BST_INT_RIGHT_CHILD_RIGHT(6);
TEST_BST_INT_RIGHT_CHILD_RIGHT(7);

TEST_GROUP_RUNNER(right_child) {
    printf("\n----------------------------\n");
    printf("Testing right_child function\n\n");

    RUN_TEST_CASE(right_child, bst_int_right_child_right_0);
    RUN_TEST_CASE(right_child, bst_int_right_child_right_1);
    RUN_TEST_CASE(right_child, bst_int_right_child_right_2);
    RUN_TEST_CASE(right_child, bst_int_right_child_right_3);
    RUN_TEST_CASE(right_child, bst_int_right_child_right_4);
    RUN_TEST_CASE(right_child, bst_int_right_child_right_5);
    RUN_TEST_CASE(right_child, bst_int_right_child_right_6);
    RUN_TEST_CASE(right_child, bst_int_right_child_right_7);
};

/* Tests for retrieve */

TEST_GROUP(retrieve);

TEST_SETUP(retrieve) {

};

TEST_TEAR_DOWN(retrieve) {
    deallocate_bst(test_bst);
    test_bst = NULL;
};

// A macro to test retrieve on example BST
#define TEST_BST_INT_RETRIEVE(THE_SIZE)                                 \
    TEST(retrieve, bst_int_retrieve_##THE_SIZE) {                       \
        test_bst = create_bst(THE_SIZE);                                \
                                                                        \
        printf("testing retrieve with example BST (elements 1 to %d)\n", \
               THE_SIZE);                                               \
                                                                        \
        for (int i = 0; i < THE_SIZE; i++) {                            \
            bst_int result = retrieve(test_bst, values[i]);             \
                                                                        \
            char *msg;                                                  \
                                                                        \
            asprintf(&msg,                                              \
                     "retrieve %d should not return NULL!",             \
                     values[i]);                                        \
                                                                        \
            TEST_ASSERT_NOT_NULL_MESSAGE(result, msg);                  \
                                                                        \
            free(msg);                                                  \
                                                                        \
            asprintf(&msg,                                              \
                     "retrieve %d should return a tree with value %d at root!", \
                     values[i], values[i]);                             \
                                                                        \
            TEST_ASSERT_EQUAL_INT_MESSAGE(value(result),                \
                                          values[i],                    \
                                          msg);                         \
                                                                        \
            free(msg);                                                  \
        }                                                               \
                                                                        \
        for (int i = THE_SIZE; i < 7; i++) {                            \
            bst_int result = retrieve(test_bst, values[i]);             \
                                                                        \
            char *msg;                                                  \
                                                                        \
            asprintf(&msg,                                              \
                     "retrieve %d should return NULL!",                 \
                     values[i]);                                        \
                                                                        \
            TEST_ASSERT_NULL_MESSAGE(result, msg);                      \
                                                                        \
            free(msg);                                                  \
        }                                                               \
                                                                        \
        char *msg;                                                      \
                                                                        \
        asprintf(&msg,                                                  \
                 "size of BST should remain equal to %d",               \
                 THE_SIZE);                                             \
                                                                        \
        TEST_ASSERT_EQUAL_INT_MESSAGE(THE_SIZE,                         \
                                      size(test_bst),                   \
                                      msg);                             \
                                                                        \
        free(msg);                                                      \
                                                                        \
        check_bst_property(test_bst, THE_SIZE);                         \
    };


TEST_BST_INT_RETRIEVE(0);
TEST_BST_INT_RETRIEVE(1);
TEST_BST_INT_RETRIEVE(2);
TEST_BST_INT_RETRIEVE(3);
TEST_BST_INT_RETRIEVE(4);
TEST_BST_INT_RETRIEVE(5);
TEST_BST_INT_RETRIEVE(6);
TEST_BST_INT_RETRIEVE(7);

TEST_GROUP_RUNNER(retrieve) {
    printf("\n-------------------------\n");
    printf("Testing retrieve function\n\n");

    RUN_TEST_CASE(retrieve, bst_int_retrieve_0);
    RUN_TEST_CASE(retrieve, bst_int_retrieve_1);
    RUN_TEST_CASE(retrieve, bst_int_retrieve_2);
    RUN_TEST_CASE(retrieve, bst_int_retrieve_3);
    RUN_TEST_CASE(retrieve, bst_int_retrieve_4);
    RUN_TEST_CASE(retrieve, bst_int_retrieve_5);
    RUN_TEST_CASE(retrieve, bst_int_retrieve_6);
    RUN_TEST_CASE(retrieve, bst_int_retrieve_7);
};

/* Tests for insert */

TEST_GROUP(insert);

TEST_SETUP(insert) {

};

TEST_TEAR_DOWN(insert) {
    deallocate_bst(test_bst);
    test_bst = NULL;
};

// A macro to test insert on an empty BST using example BST
#define TEST_BST_INT_INSERT_EX(THE_SIZE, INSERT_FUNCTION)                      \
    TEST(insert, bst_int_##INSERT_FUNCTION##_ex_##THE_SIZE) {           \
        test_bst = nil();                                               \
                                                                        \
        printf("testing %s with empty BST, inserting %d example nodes in order\n", \
               #INSERT_FUNCTION, THE_SIZE);                             \
                                                                        \
        for (int i = 0; i < THE_SIZE; i++) {                            \
            test_bst = INSERT_FUNCTION(test_bst, values[i]);            \
            check_bst_property(test_bst, i + 1);                        \
        }                                                               \
                                                                        \
        for (int i = 0; i < THE_SIZE; i++) {                            \
            TEST_ASSERT_NOT_NULL(retrieve(test_bst, values[i]));        \
        }                                                               \
                                                                        \
        char *msg;                                                      \
                                                                        \
        asprintf(&msg, "the size of the tree should be %d", THE_SIZE);  \
                                                                        \
        TEST_ASSERT_EQUAL_INT_MESSAGE(THE_SIZE, my_size(test_bst),      \
                                      msg);                             \
                                                                        \
        free(msg);                                                      \
    };

// A macro to test insert on an empty BST (random insertions)
#define TEST_BST_INT_INSERT_RANDOM(THE_SIZE, INSERT_FUNCTION)           \
    TEST(insert, bst_int_##INSERT_FUNCTION##_random_##THE_SIZE) {       \
        test_bst = nil();                                               \
                                                                        \
        printf("testing %s with empty BST, inserting %d random different elements)\n",  \
               #INSERT_FUNCTION, THE_SIZE);                             \
                                                                        \
        int tree_array[THE_SIZE];                                       \
        int val_array[1000] = { [0 ... 999 ] = 0 };                     \
        int real_size = 0;                                              \
        srand(time(NULL));                                              \
                                                                        \
        for (int i = 0; i < THE_SIZE; i++) {                            \
            tree_array[i] = rand() % 1000;                              \
                                                                        \
            if (! val_array[tree_array[i]]) {                           \
                real_size++;                                            \
                val_array[tree_array[i]] = 1;                           \
            }                                                           \
                                                                        \
            test_bst = INSERT_FUNCTION(test_bst, tree_array[i]);        \
            check_bst_property(test_bst, real_size);                    \
        }                                                               \
                                                                        \
        for (int i = 0; i < THE_SIZE; i++) {                            \
            TEST_ASSERT_NOT_NULL(retrieve(test_bst, tree_array[i]));    \
        }                                                               \
                                                                        \
        char *msg;                                                      \
                                                                        \
        asprintf(&msg, "the size of the tree should be %d", real_size); \
                                                                        \
        TEST_ASSERT_EQUAL_INT_MESSAGE(real_size, my_size(test_bst),     \
                                      msg);                             \
                                                                        \
        free(msg);                                                      \
    };

TEST_BST_INT_INSERT_EX(0, insert);
TEST_BST_INT_INSERT_EX(1, insert);
TEST_BST_INT_INSERT_EX(2, insert);
TEST_BST_INT_INSERT_EX(3, insert);
TEST_BST_INT_INSERT_EX(4, insert);
TEST_BST_INT_INSERT_EX(5, insert);
TEST_BST_INT_INSERT_EX(6, insert);
TEST_BST_INT_INSERT_EX(7, insert);
TEST_BST_INT_INSERT_RANDOM(0, insert);
TEST_BST_INT_INSERT_RANDOM(1, insert);
TEST_BST_INT_INSERT_RANDOM(2, insert);
TEST_BST_INT_INSERT_RANDOM(3, insert);
TEST_BST_INT_INSERT_RANDOM(4, insert);
TEST_BST_INT_INSERT_RANDOM(5, insert);
TEST_BST_INT_INSERT_RANDOM(6, insert);
TEST_BST_INT_INSERT_RANDOM(7, insert);
TEST_BST_INT_INSERT_RANDOM(100, insert);

#ifndef STD
TEST_BST_INT_INSERT_EX(0, insert_using_pointers);
TEST_BST_INT_INSERT_EX(1, insert_using_pointers);
TEST_BST_INT_INSERT_EX(2, insert_using_pointers);
TEST_BST_INT_INSERT_EX(3, insert_using_pointers);
TEST_BST_INT_INSERT_EX(4, insert_using_pointers);
TEST_BST_INT_INSERT_EX(5, insert_using_pointers);
TEST_BST_INT_INSERT_EX(6, insert_using_pointers);
TEST_BST_INT_INSERT_EX(7, insert_using_pointers);
TEST_BST_INT_INSERT_RANDOM(0, insert_using_pointers);
TEST_BST_INT_INSERT_RANDOM(1, insert_using_pointers);
TEST_BST_INT_INSERT_RANDOM(2, insert_using_pointers);
TEST_BST_INT_INSERT_RANDOM(3, insert_using_pointers);
TEST_BST_INT_INSERT_RANDOM(4, insert_using_pointers);
TEST_BST_INT_INSERT_RANDOM(5, insert_using_pointers);
TEST_BST_INT_INSERT_RANDOM(6, insert_using_pointers);
TEST_BST_INT_INSERT_RANDOM(7, insert_using_pointers);
TEST_BST_INT_INSERT_RANDOM(100, insert_using_pointers);
#endif

TEST_GROUP_RUNNER(insert) {
    printf("\n-------------------------\n");
    printf("Testing insert function\n\n");

    RUN_TEST_CASE(insert, bst_int_insert_ex_0);
    RUN_TEST_CASE(insert, bst_int_insert_ex_1);
    RUN_TEST_CASE(insert, bst_int_insert_ex_2);
    RUN_TEST_CASE(insert, bst_int_insert_ex_3);
    RUN_TEST_CASE(insert, bst_int_insert_ex_4);
    RUN_TEST_CASE(insert, bst_int_insert_ex_5);
    RUN_TEST_CASE(insert, bst_int_insert_ex_6);
    RUN_TEST_CASE(insert, bst_int_insert_ex_7);
    RUN_TEST_CASE(insert, bst_int_insert_random_0);
    RUN_TEST_CASE(insert, bst_int_insert_random_1);
    RUN_TEST_CASE(insert, bst_int_insert_random_2);
    RUN_TEST_CASE(insert, bst_int_insert_random_3);
    RUN_TEST_CASE(insert, bst_int_insert_random_4);
    RUN_TEST_CASE(insert, bst_int_insert_random_5);
    RUN_TEST_CASE(insert, bst_int_insert_random_6);
    RUN_TEST_CASE(insert, bst_int_insert_random_7);
    RUN_TEST_CASE(insert, bst_int_insert_random_100);

#ifndef STD
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_ex_0);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_ex_1);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_ex_2);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_ex_3);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_ex_4);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_ex_5);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_ex_6);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_ex_7);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_random_0);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_random_1);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_random_2);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_random_3);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_random_4);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_random_5);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_random_6);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_random_7);
    RUN_TEST_CASE(insert, bst_int_insert_using_pointers_random_100);
#endif
};

/* Tests for delete */

TEST_GROUP(delete);

TEST_SETUP(delete) {

};

TEST_TEAR_DOWN(delete) {
    //deallocate_bst(test_bst);
    test_bst = NULL;
};

// A macro to test delete on the example BST
#define TEST_BST_INT_DELETE(THE_SIZE, DELETE_FUNCTION)                  \
    TEST(delete, bst_int_##DELETE_FUNCTION##_##THE_SIZE) {              \
        test_bst = create_bst(THE_SIZE);                                \
                                                                        \
        printf("testing %s with example BST, deleting %dth element)\n", \
               #DELETE_FUNCTION, THE_SIZE);                             \
                                                                        \
        for (int i = 0; i < THE_SIZE; i++) {                            \
            test_bst = DELETE_FUNCTION(test_bst, values[i]);            \
            check_bst_property(test_bst, THE_SIZE - i - 1);             \
            TEST_ASSERT_NULL(retrieve(test_bst, values[i]));            \
            TEST_ASSERT_EQUAL_INT(THE_SIZE - i - 1,                     \
                                  my_size(test_bst));                   \
        }                                                               \
    };

TEST_BST_INT_DELETE(0, delete);
TEST_BST_INT_DELETE(1, delete);
TEST_BST_INT_DELETE(2, delete);
TEST_BST_INT_DELETE(3, delete);
TEST_BST_INT_DELETE(4, delete);
TEST_BST_INT_DELETE(5, delete);
TEST_BST_INT_DELETE(6, delete);
TEST_BST_INT_DELETE(7, delete);

#ifndef STD
TEST_BST_INT_DELETE(0, delete_using_pointers);
TEST_BST_INT_DELETE(1, delete_using_pointers);
TEST_BST_INT_DELETE(2, delete_using_pointers);
TEST_BST_INT_DELETE(3, delete_using_pointers);
TEST_BST_INT_DELETE(4, delete_using_pointers);
TEST_BST_INT_DELETE(5, delete_using_pointers);
TEST_BST_INT_DELETE(6, delete_using_pointers);
TEST_BST_INT_DELETE(7, delete_using_pointers);

#endif
TEST_GROUP_RUNNER(delete) {
    printf("\n-----------------------\n");
    printf("Testing delete function\n\n");

    RUN_TEST_CASE(delete, bst_int_delete_0);
    RUN_TEST_CASE(delete, bst_int_delete_1);
    RUN_TEST_CASE(delete, bst_int_delete_2);
    RUN_TEST_CASE(delete, bst_int_delete_3);
    RUN_TEST_CASE(delete, bst_int_delete_4);
    RUN_TEST_CASE(delete, bst_int_delete_5);
    RUN_TEST_CASE(delete, bst_int_delete_6);
    RUN_TEST_CASE(delete, bst_int_delete_7);
#ifndef STD

    RUN_TEST_CASE(delete, bst_int_delete_using_pointers_0);
    RUN_TEST_CASE(delete, bst_int_delete_using_pointers_1);
    RUN_TEST_CASE(delete, bst_int_delete_using_pointers_2);
    RUN_TEST_CASE(delete, bst_int_delete_using_pointers_3);
    RUN_TEST_CASE(delete, bst_int_delete_using_pointers_4);
    RUN_TEST_CASE(delete, bst_int_delete_using_pointers_5);
    RUN_TEST_CASE(delete, bst_int_delete_using_pointers_6);
    RUN_TEST_CASE(delete, bst_int_delete_using_pointers_7);
#endif
};

/* run functions */

static void runNilTests(void) {
    RUN_TEST_GROUP(nil);
}

static void runIsEmptyTests(void) {
    RUN_TEST_GROUP(is_empty);
}

static void runSizeTests(void) {
    RUN_TEST_GROUP(size);
}

static void runValueTests(void) {
    RUN_TEST_GROUP(value);
}

static void runHeightTests(void) {
    RUN_TEST_GROUP(height);
}

static void runLeftChildTests(void) {
    RUN_TEST_GROUP(left_child);
}

static void runRightChildTests(void) {
    RUN_TEST_GROUP(right_child);
}

static void runRetrieveTests(void) {
    RUN_TEST_GROUP(retrieve);
}

static void runInsertTests(void) {
    RUN_TEST_GROUP(insert);
}

static void runDeleteTests(void) {
    RUN_TEST_GROUP(delete);
}

static void runAllTests(void) {
    RUN_TEST_GROUP(nil);
    RUN_TEST_GROUP(is_empty);
    RUN_TEST_GROUP(size);
    RUN_TEST_GROUP(value);
    RUN_TEST_GROUP(height);
    RUN_TEST_GROUP(left_child);
    RUN_TEST_GROUP(right_child);
    RUN_TEST_GROUP(retrieve);
    RUN_TEST_GROUP(insert);
    RUN_TEST_GROUP(delete);
}

/* complexity tests */

// test for worst case complexity
#define MAX_NB_OF_NODES 51200

double time_to_insert_worst_case(bst_int *p_tree, int nb) {
    clock_t end;
    clock_t start = clock();

    for (int i = 0; i < nb; i++) {
        *p_tree = insert(*p_tree, i);
    }

    end = clock();

    assert (size(*p_tree) == nb);

    return (double) (end - start) / CLOCKS_PER_SEC;
}

double time_to_delete_worst_case(bst_int *p_tree, int nb) {
    clock_t end;
    clock_t start = clock();

    for (int i = nb - 1; i >= 0; i--) {
        *p_tree = delete(*p_tree, i);
    }

    end = clock();

    assert (size(*p_tree) == 0);

    return (double) (end - start) / CLOCKS_PER_SEC;
}

bool test_complexity_worst_case() {
    if (access("./results/", W_OK) == -1) {
        error(1, 0, "directory results does not exist!\n");

        return false;
    }

    printf("  starting tests for worst case\n");

    FILE *p_file = fopen("./results/data_complexity_bst_worst_case.csv", "w");

    bst_int t = nil();

    int nb     = 100;
    double ins = 0.0;
    double ext = 0.0;

    while (nb <= MAX_NB_OF_NODES) {
        printf("    test with %d nodes: ", nb);
        printf("inserting... ");
        fflush(stdout);
        ins = time_to_insert_worst_case(&t, nb);
        printf("deleting... ");
        fflush(stdout);
        ext = time_to_delete_worst_case(&t, nb);
        printf("OK!\n");
        fprintf(p_file, "%d,%f,%f\n", nb, ins, ext);
        nb *= 2;
    }

    fclose(p_file);

    return true;
}

// test for complexity test with random values
int chosen[MAX_NB_OF_NODES];

double time_to_insert_random(bst_int *p_tree, int nb) {
    if (nb == 0) {
        return 0.0;
    }

    srand(time(NULL));

    for (int i = 0; i < MAX_NB_OF_NODES; i++) {
        chosen[i] = 0;
    }

    clock_t end;
    clock_t start;
    long    amount    = 0;
    int     orig_size = nb;

    while (nb != 0) {
        int n = rand() % MAX_NB_OF_NODES;

        if (! chosen[n]) {
            chosen[n] = 1;
            start = clock();
            *p_tree = insert(*p_tree, n);
            end = clock();
            amount += (end - start);
            nb--;
        }
    }

    assert (size(*p_tree) == orig_size);

    return (double) amount / CLOCKS_PER_SEC;
}

double time_to_delete_random(bst_int *p_tree, int nb) {
    if (nb == 0) {
        return 0.0;
    }

    srand(time(NULL));

    clock_t end;
    clock_t start;
    long    amount = 0;

    while (nb != 0) {
        int n = rand() % MAX_NB_OF_NODES;

        if (chosen[n]) {
            chosen[n] = 0;
            start = clock();
            *p_tree = delete(*p_tree, n);
            end = clock();
            amount += (end - start);
            nb--;
        }
    }

    assert (size(*p_tree) == 0);

    return (double) amount / CLOCKS_PER_SEC;
}

bool test_complexity_random() {
    if (access("./results/", W_OK) == -1) {
        error(1, 0, "directory results does not exist!\n");

        return false;
    }

    printf("  starting tests with random values\n");

    FILE *p_file = fopen("./results/data_complexity_bst_random.csv", "w");

    bst_int t = nil();

    int nb     = 100;
    double ins = 0.0;
    double ext = 0.0;

    while (nb <= MAX_NB_OF_NODES) {
        printf("    test with %d nodes: ", nb);
        printf("inserting... ");
        fflush(stdout);
        ins = time_to_insert_random(&t, nb);
        printf("deleting... ");
        fflush(stdout);
        ext = time_to_delete_random(&t, nb);
        printf("OK!\n");
        fprintf(p_file, "%d,%f,%f\n", nb, ins, ext);
        nb *= 2;
    }

    fclose(p_file);

    return true;
}


void run_complexity_tests() {
    test_complexity_worst_case();
    test_complexity_random();
}

/* main */

int main(int argc, char * argv[]) {
    int  c;
    bool has_passed_test = false;

    while (1) {
        static struct option long_options[] = {
            {"nil",            no_argument, 0, 'N'},
            {"is_empty",       no_argument, 0, 'E'},
            {"size",           no_argument, 0, 'S'},
            {"height",         no_argument, 0, 'H'},
            {"value",          no_argument, 0, 'V'},
            {"left_child",     no_argument, 0, 'L'},
            {"right_child",    no_argument, 0, 'R'},
            {"retrieve",       no_argument, 0, 'r'},
            {"insert",         no_argument, 0, 'I'},
            {"delete",         no_argument, 0, 'D'},
            {"all",            no_argument, 0, 'A'},
            {"complexity",     no_argument, 0, 'C'},
            {0, 0, 0, 0}
        };

        int option_index = 0;

        c = getopt_long_only(argc, argv, "NESHVLRrIDAC",
                             long_options, &option_index);

        /* Detect the end of the options. */
        if (c == -1)
            break;

        switch (c) {
        case 'N':
            printf("starting tests for nil\n");
            UnityMain(argc, (const char **) argv, runNilTests);
            has_passed_test = true;
            break;

        case 'E':
            printf("starting tests for is_empty\n");
            UnityMain(argc, (const char **) argv, runIsEmptyTests);
            has_passed_test = true;
            break;

        case 'S':
            printf("starting tests for size\n");
            UnityMain(argc, (const char **) argv, runSizeTests);
            has_passed_test = true;
            break;

        case 'V':
            printf("starting tests for value\n");
            UnityMain(argc, (const char **) argv, runValueTests);
            has_passed_test = true;
            break;

        case 'H':
            printf("starting tests for height\n");
            UnityMain(argc, (const char **) argv, runHeightTests);
            has_passed_test = true;
            break;

        case 'L':
            printf("starting tests for left_child\n");
            UnityMain(argc, (const char **) argv, runLeftChildTests);
            has_passed_test = true;
            break;

        case 'R':
            printf("starting tests for right_child\n");
            UnityMain(argc, (const char **) argv, runRightChildTests);
            has_passed_test = true;
            break;

        case 'r':
            printf("starting tests for retrieve\n");
            UnityMain(argc, (const char **) argv, runRetrieveTests);
            has_passed_test = true;
            break;

        case 'I':
            printf("starting tests for insert\n");
            UnityMain(argc, (const char **) argv, runInsertTests);
            has_passed_test = true;
            break;

        case 'D':
            printf("starting tests for delete\n");
            UnityMain(argc, (const char **) argv, runDeleteTests);
            has_passed_test = true;
            break;

        case 'A':
            printf("starting all tests\n");
            UnityMain(argc, (const char **) argv, runAllTests);
            has_passed_test = true;
            break;

        case 'C':
            printf("starting complexity tests\n");
            run_complexity_tests();
            has_passed_test = true;
            break;

        default:
            break;
        }
    }

    if (! has_passed_test) {
        error(1, 0, "\nusage: test-bst-int [--nil|--is_empty|--size|--height|"
              "--left_child|--right_child|--retrieve|--insert|--delete|--all]\n"
              "    --all starts all tests\n"
              "    --complexity starts complexity tests. Results should be in CSV files in results directory\n"
              "    other options only start tests on the specified function\n"
              "    options can also be chosen by their first letter: -N, -I etc.\n"
            );
    }
}
