#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "file_util.h"

long problem_one(char* content) {
    size_t i = 0;
    char ch;
    long sum = 0;

    while ((ch = content[i++]) != '\0') {
        char next = content[i] == '\0' ? content[0] : content[i];

        if (ch == next) {
            sum += (ch - '0');
        }
    }

    return sum;
}

long problem_two(char* content) {
    size_t i = 0;
    char ch;
    long sum = 0;
    size_t length = strlen(content);

    while ((ch = content[i++]) != '\0') {
        char next = content[((i - 1) + length / 2) % length];

        if (ch == next) {
            sum += (ch - '0');
        }
    }

    return sum;
}

int main() {
    char* content = string_input("1");
    printf("%ld\n", problem_one(content));
    printf("%ld\n", problem_two(content));
    free(content);

    return 0;
}
