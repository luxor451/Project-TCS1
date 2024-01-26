#include <stdio.h>
#include <stdlib.h>

struct complex {
    double real;
    double imag;
};

struct complex conjugate(struct complex x);
struct complex mult(struct complex x, struct complex y);
double sqr_norm(struct complex x);
void print_complex(struct complex x);

struct complex conjugate(struct complex x) {
    struct complex res = {
        .real = x.real,
        .imag = -x.imag
    };

    return res;
}

struct complex mult(struct complex x, struct complex y) {
    struct complex res = {
        .real = x.real * y.real - x.imag * y.imag,
        .imag = x.real * y.imag + x.imag * y.real
    };

    return res;
}

double sqr_norm(struct complex x) {
    return mult(x, conjugate(x)).real;
}

void print_complex(struct complex x) {
    printf("%f %s %f i",
           x.real,
           x.imag < 0 ? "-" : "+",
           x.imag < 0 ? - x.imag : x.imag);
}

