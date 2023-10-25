#include <limits.h>
#include <regex.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "file_util.h"
#include "regex_util.h"

int checksum(int (*rowsum)(int[]), regex_t* re) {
    char* input = string_input("2");
    char* line = strtok(input, "\n");
    regmatch_t pmatch[1];
    int sum = 0;
    int cols[16];

    while (line != NULL) {
        for (int col = 0;; col++) {
            if (regexec(re, line, 1, pmatch, 0)) {
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

        sum += rowsum(cols);
        line = strtok(NULL, "\n");
    }

    free(input);
    return sum;
}

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

int main() {
    REGEX(re, "[[:digit:]]+", REG_EXTENDED);

    printf("%d\n", checksum(&problem_one, &re));
    printf("%d\n", checksum(&problem_two, &re));

    regfree(&re);
    return 0;
}
