/**
 * A simple program to load a maze configuration file
 * and "print" it on the console
 *
 * Ex. of usage:
 *
 * ./app-ex-loader ./data/maze.txt
 */

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    // testing if a filename has been given
    if (argc != 2) {
        fprintf(stderr, "You must provide a file name!\n");
        exit(EXIT_FAILURE);
    }

    // open file. The filename is the first argument on the command
    // line, hence stored in argv[1]
    FILE *p_file = NULL;

    p_file = fopen(argv[1], "r");

    if (p_file == NULL) {
        fprintf(stderr, "Cannot read file %s!\n", argv[1]);
        exit(EXIT_FAILURE);
    }

    // read dimensions on first line
    int width         = 0;
    int height        = 0;
    int fscanf_result = 0;

    fscanf_result = fscanf(p_file, "%d %d\n", &width, &height);

    if (fscanf_result != 2) {
        fprintf(stderr, "First line is not syntactically correct!\n");
        exit(EXIT_FAILURE);
    }

    printf("map dimensions: %d x %d\n", width, height);

    // read following lines
    // each line is composed of two integer values
    int below = 0;
    int right = 0;

    for (int i = 0; i < height * width; i++) {
        fscanf_result = fscanf(p_file, "%d %d",
                               &below, &right);

        if (fscanf_result != 2) {
            fprintf(stderr, "Line %d is not syntactically correct!\n", i + 2);
            exit(EXIT_FAILURE);
        }

        printf("line %4d-> { below: %d, right: %d }\n",
               i + 2, below, right);
    }

    fclose(p_file);

    return EXIT_SUCCESS;
}
