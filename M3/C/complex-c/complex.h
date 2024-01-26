#ifndef COMPLEX_H
#define COMPLEX_H


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


#endif