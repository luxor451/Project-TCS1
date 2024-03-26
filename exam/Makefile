CC = gcc
CFLAGS = -g -std=c99 -Wall
FILE = queue_int
OBJ_FILES = $(FILE).o test_utils.o

.PHONY: clean all doc

%.o: %.c
	$(CC) $(CFLAGS) -o $@ -c $^

doc:
	doxygen conf/doxygen.conf
	firefox doc/html/files.html &

check-expression: test-expression.c
	$(CC) -std=c99 -c test-expression.c

check-syntax: $(FILE).o

create: $(OBJ_FILES) test-create.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-create

size: $(OBJ_FILES) test-size.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-size

is-empty: $(OBJ_FILES) test-is-empty.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-is-empty

enqueue: $(OBJ_FILES) test-enqueue.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-enqueue

dequeue: $(OBJ_FILES) test-dequeue.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-dequeue

copy: $(OBJ_FILES) test-copy.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-copy

clean:
	rm -f *.o test-create test-is-empty test-size test-enqueue \
	test-dequeue test-append test-copy

all: create is-empty size enqueue dequeue copy
