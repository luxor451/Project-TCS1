#include <stdio.h>

int inc(int i);
void pretty_print(int i);
int add(int i, int j);

int inc(int i) {
    return i + 1;
}

void pretty_print(int i) {
    printf("Here is %d\n", i);
}

int add(int i, int j) {
    return i + j;
}

int main(void) {
    int    i = 1;
    double j = 3.5;
    int    z = 0;

    pretty_print(i);

    printf("inc(%d) = %d\n", i, inc(i));

    printf("Now, an addition!\n");

    z = add(i, j);
    pretty_print(z);

    return 0;
}
