CC = gcc
CFLAGS = -g -std=c99 -Wall
FILE = grading

.PHONY: clean all doc

%.o: %.c
	$(CC) $(CFLAGS) -o $@ -c $^

doc:
	doxygen conf/doxygen.conf
	firefox doc/html/index.html

check-syntax: $(FILE).o

create-promo: grading.o test-create-promo.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-create-promo

create-student: grading.o test-create-student.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-create-student

grade: grading.o test-grade.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-grade

add-in-pc: grading.o test-add-in-pc.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-add-in-pc

nb-of-students: grading.o test-nb-of-students.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-nb-of-students

best-in-promo: grading.o test-best-in-promo.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-best-in-promo

sort-by-grade: grading.o test-sort-by-grade.o
	$(CC) $(CFLAGS) -o test-$@ $^
	./test-sort-by-grade

clean:
	rm -f *.o test-create-promo test-create-student test-grade \
	test-add-in-pc test-nb-of-students test-best-in-promo \
	test-sort-by-grade

all: create-student grade add-in-pc nb-of-students best-in-promo sort-by-grade
