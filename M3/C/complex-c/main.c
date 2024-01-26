#include "complex.h"

int main(void) {
    struct complex x1 = { .real = -1.0, .imag = -76.0 };
    struct complex x2 = { .real = 1.0, .imag = 1024.0 };

    printf("x1 before operations: ");
    print_complex(x1);
    printf("\n");

    x1 = mult(conjugate(x1), x2);

    printf("x1 after operations: ");
    print_complex(x1);
    printf("\n");

    return EXIT_SUCCESS;
}
