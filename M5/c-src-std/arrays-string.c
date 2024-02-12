#include <stdio.h>

void print_chars(char string[]);

void print_chars(char string[]) {
    for (int i = 0; string[i] != '\0'; i++) {
        printf("%c\n", string[i]);
    }
}

int main(void) {
    char my_string[] = "I love C";

    print_chars(my_string);

    return 0;
}
