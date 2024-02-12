CC = gcc
CFLAGS = -std=c99 -Wall -Werror

.PHONY: run

pascal: pascal.c pascal.h prog-pascal.c
	$(CC) $(CFLAGS) -o pascal pascal.c prog-pascal.c

run: pascal
	./pascal