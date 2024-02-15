/**
 * @file example-string.c
 *
 * @brief Simple program to explain how to create
 *        strings with variable values
 *
 * @author C. Garion
 *
 */

#include <stdio.h>
#include <string.h>

int main() {
    char *original = "simulation";
    int   length   = strlen(original);

    // declaring a string of length length + 6 characters + the final
    // \0 character
    char my_string[length + 7];
    sprintf(my_string, "simulation-%d.%s", 3, "txt");

    printf("The new string: %s\n", my_string);

    return 0;
}
