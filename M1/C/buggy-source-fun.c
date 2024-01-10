#include <stdio.h>

int inc(int i);
pretty_print(int i);
void add(int i, int j);

int inc(int i) {
    return i + 1;
}

pretty_print(int i) {
    printf("Here is %d\n", i);
}

void add(int i, int j) {
    printf("%d\n", i + j);
}

int main(void) {
    int    i = 1;
    double j = 3.5;
    int    z = 0;

    pretty_print(i);

    printf("inc(%d) = %d\n", i, incr(i));

    pretty_print("Now, an addition!");

    z = add(i, j);

    return 0;
}
