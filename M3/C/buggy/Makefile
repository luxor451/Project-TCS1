CC = gcc
CFLAGS = -std=c99 -Wall -Werror


.PHONY: clean


.DEFAULT_GOAL := prog-grade

prog-grade: prog-grade.c grade.o dfi.o
	$(CC) $(CFLAGS) -c -o grade.o grade.c
	$(CC) $(CFLAGS) -c -o dfi.o dfi.c
	$(CC) $(CFLAGS) -o $@ $@.c grade.o dfi.o
	./prog-grade

clean:
	rm -f *.o
	rm -f prog-grade prog_grade.o