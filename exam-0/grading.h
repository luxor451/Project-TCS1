/**
 * @file grading.h
 *
 * @author C. Garion
 */
#define MAX_STUDENTS_NB 24

typedef struct student student;

struct student {
    char   *first_name;
    char   *last_name;
    double grade;
};

typedef struct pc pc;

struct pc {
    int      nb_students;
    student *students_tab[MAX_STUDENTS_NB];
};

typedef struct promo promo;

struct promo {
    int year;
    pc  pcs_tab[8];
};

/**
 * Create a new promo
 *
 * @param year  the year of the promo
 *
 * @return a pointer to a dynamically created promo
 */
promo *create_promo(int year);

/**
 * Create a new student
 *
 * @param first_name  the first name of the student
 * @param last_name   the last name of the student
 *
 * @return a pointer to a dynamically created student
 *
 * @post The created student should have -1 for grade.
 */
student *create_student(char *first_name, char *last_name);

/**
 * Grade a student
 *
 * @param p_student  a pointer to the student to grade
 * @param grade      the grade of the student
 *
 * @post After a call to the function, the grade associated
 *       to the student is the one passed as parameter.
 */
void grade(student *p_student, double grade);

/**
 * Add a student in a PC
 *
 * @param p_student  a pointer to the student to add
 * @param p_pc       the PC
 *
 * @post After a call to the function, the student is added
 *       in the PC
 */
void add_in_pc(student *p_student, pc *p_pc);

/**
 * Get the number of students in the promo
 *
 * @param p_promo  a pointer to the promo
 *
 * @returns the number of students in the promo
 */
int nb_of_students(promo *p_promo);

/**
 * Get the best student
 *
 * @param p_promo  a pointer to the promo
 *
 * @returns a pointer to the best student of the promo
 */
student *best_in_promo(promo *p_promo);

/**
 * Get a tab with all students sorted by grades
 *
 * @param p_promo  a pointer to the promo
 *
 * @returns an array of pointer to students with students sorted by grades
 */
student **sort_by_grade(promo *p_promo);
