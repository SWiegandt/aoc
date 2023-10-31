#include <stdio.h>
#include <string.h>
#include "file_util.h"

long problem_one(const char* content) {
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

long problem_two(const char* content) {
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
    FileInput* content = read_input(1);
    printf("%ld\n", problem_one(content->input));
    printf("%ld\n", problem_two(content->input));
    cleanup(content);

    return 0;
}
