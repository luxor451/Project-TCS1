CC = gcc
CFLAGS = -std=c99 -Wall -g

check-syntax: linked_list_int.h linked_list_int.c app-linked-list-int.c
	$(CC) $(CFLAGS) -fsyntax-only linked_list_int.h linked_list_int.c app-linked-list-int.c

app-linked-list-int: linked_list_int.h linked_list_int.c app-linked-list-int.c
	$(CC) $(CFLAGS) -o app-linked-list-int linked_list_int.c app-linked-list-int.c

run: app-linked-list-int
	./app-linked-list-int

valgrind: app-linked-list-int
	valgrind --leak-check=full ./app-linked-list-int
