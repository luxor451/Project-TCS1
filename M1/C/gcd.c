#include <stdio.h>

int gcd(int a, int b);

int gcd(int a, int b) {
    int x = a;
    int y = b;

    while (x != y) {
        if (x > y) {
           x = x - y;
        } else {
           y = y - x;
        }
    }

    return x;
}

int main(void) {
    int i1 = 42;
    int i2 = 1024;

    printf("The GCD of %d and %d is %d\n",
           i1, i2, gcd(i1, i2));

    return 0;
}
