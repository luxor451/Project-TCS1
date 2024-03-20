# these two variables set paths for Unity at ISAE SI. You may have to
# change them if you want to use your personal computer.
UNITYLIB = /usr/local/Unity/lib
UNITYINC = /usr/local/Unity/include/

CC = gcc
CFLAGS = -std=c99 -Wall -g -I ./include -I $(UNITYINC)
LDFLAGS = -L $(UNITYLIB) -l unity

.PHONY: init clean check-syntax run valgrind tests \
	all nil is_empty size height value left_child right_child retrieve insert delete complexity
.DEFAULT_GOAL := all

doc: conf/doxygen.conf
	doxygen $<
	firefox doc/html/index.html

check-syntax: src/binary_search_tree_int.c src/app-bst-int.c
	$(CC) $(CFLAGS) -c $^

app-bst-int: src/binary_search_tree_int.c src/app-bst-int.c
	$(CC) $(CFLAGS) -o $@ $^

run: app-bst-int
	./app-bst-int

valgrind: app-bst-int
	valgrind --leak-check=full ./app-bst-int

tests:	tests/binary_search_tree_int_tests.c src/binary_search_tree_int.c
	$(CC) $(CFLAGS) -o tests-bst $^ $(LDFLAGS)

clean:
	rm -Rf *.o ./app-bst-int ./tests-bst

all nil is_empty size height value left_child right_child retrieve insert delete complexity: tests
	LD_LIBRARY_PATH=$$LD_LIBRARY_PATH:$(UNITYLIB) ./tests-bst --$@
