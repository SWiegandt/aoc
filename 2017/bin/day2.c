#include <limits.h>
#include <regex.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "file_util.h"
#include "regex_util.h"

typedef struct {
    int one;
    int two;
} Sums;

int problem_one(int cols[]) {
    int max = 0, min = INT_MAX;

    for (int col = 0; col < 16; col++) {
        int number = cols[col];

        if (number > max) {
            max = number;
        }

        if (number < min) {
            min = number;
        }
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
    REGEX(re, "[[:digit:]]+", REG_EXTENDED);
    char* input = string_input("2");
    char* line = strtok(input, "\n");
    regmatch_t pmatch[1];
    Sums sums;
    int cols[16];

    while (line != NULL) {
        for (int col = 0;; col++) {
            if (regexec(&re, line, 1, pmatch, 0)) {
                break;
            }

            int len = pmatch[0].rm_eo - pmatch[0].rm_so;
            char* match = calloc(len + 1, 1);
            line += pmatch[0].rm_so;
            snprintf(match, len + 1, "%s", line);
            line += len;

            cols[col] = atoi(match);
            free(match);
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
