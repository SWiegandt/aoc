#include <limits.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include "file_util.h"
#include "regex_util.h"

typedef struct {
    int one;
    int two;
} Sums;

int problem_one(int cols[]) {
    long max = 0, min = LONG_MAX;

    for (int col = 0; col < 16; col++) {
        min = fminl(min, cols[col]);
        max = fmaxl(max, cols[col]);
    }

    return max - min;
}

int problem_two(int cols[]) {
    for (int i = 0; i < 16; i++) {
        for (int j = i + 1; j < 16; j++) {
            if (cols[i] % cols[j] == 0) {
                return cols[i] / cols[j];
            }

            if (cols[j] % cols[i] == 0) {
                return cols[j] / cols[i];
            }
        }
    }

    return 0;
}

Sums checksum() {
    Sums sums = {0, 0};
    regex_t re;

    if (re_compile(&re, "[0-9]+", REG_EXTENDED)) {
        exit(1);
    }

    char* input = string_input(2);
    char* line = strtok(input, "\n");
    regmatch_t pmatch[1];
    int cols[16];

    while (line != NULL) {
        for (int col = 0;; col++) {
            if (regexec(&re, line, 1, pmatch, 0)) {
                break;
            }

            char* match = re_group(line, pmatch, 0);
            cols[col] = atoi(match);
            free(match);
            re_next(&line, pmatch);
        }

        sums.one += problem_one(cols);
        sums.two += problem_two(cols);
        line = strtok(NULL, "\n");
    }

    free(input);
    regfree(&re);

    return sums;
}

int main() {
    Sums sums = checksum();
    printf("%d\n", sums.one);
    printf("%d\n", sums.two);

    return 0;
}
