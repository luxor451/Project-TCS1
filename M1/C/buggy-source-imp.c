#include <stdbool.h>
#include <math.h>
#include <stdio.h>

/* This simple program should compute the square root of 2 using
 * Newton's method with a precision of 1E-6. */

int main(void) {

    int   max_number_of_iterations = 20;
    float epsilon                  = 1E-6;
    bool  solution                 = false;
    float x0                       = 2.0;
    float x1;

    for (int i = 1; i <= max_number_of_iterations; i++) {
        float y      = x0 * x0 - 2.0;
        float yprime = 2 * x0;
        x1 = x0 - y / yprime;

        if (fabs(x1 - x0) <= epsilon * fabs(x1)) {
            solution = true;
            break;
        }

        x0 = x1;
    }

    if (solution) {
        printf("sqrt(2) = %f\n", x1);
    } else {
        printf("no convergence in %d iterations...\n", max_number_of_iterations);
    }

    return 0;
}
