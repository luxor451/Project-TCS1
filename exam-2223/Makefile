CC = gcc
CFLAGS = -g -std=c99 -Wall
FILE = ring_list
OBJ_FILES = $(FILE).o test_utils.o

.PHONY: clean all doc

%.o: %.c
	$(CC) $(CFLAGS) -o $@ -c $^

doc:
	doxygen conf/doxygen.conf
	firefox doc/html/index.html &

check-syntax: $(FILE).o

create: $(OBJ_FILES) test-create.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-create

is-empty: $(OBJ_FILES) test-is-empty.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-is-empty

insert: $(OBJ_FILES) test-insert.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-insert

rotate: $(OBJ_FILES) test-rotate.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-rotate

size: $(OBJ_FILES) test-size.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-size

append: $(OBJ_FILES) test-append.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-append

clean:
	rm -f *.o test-create test-is-empty test-insert test-rotate test-size test-append

all: create is-empty insert rotate size append
