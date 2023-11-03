#include <stdlib.h>
#include "file_util.h"

#define JUMPS 1052

int problem_one(int jumps[]) {
    int steps = 0;

    for (int j = 0; j >= 0 && j < JUMPS;) {
        int jump = jumps[j];
        jumps[j]++;
        j += jump;
        steps++;
    }

    return steps;
}

int problem_two(int jumps[]) {
    int steps = 0;

    for (int j = 0; j >= 0 && j < JUMPS;) {
        int jump = jumps[j];
        jumps[j] += 2 * (jump < 3) - 1;
        j += jump;
        steps++;
    }

    return steps;
}

int main() {
    FileInput* input = read_input(5);
    int jumps_one[JUMPS];
    int jumps_two[JUMPS];
    int i = 0;

    loop_file(input, line) {
        jumps_one[i] = jumps_two[i] = atoi(line);
        i++;
    }

    printf("%d\n", problem_one(jumps_one));
    printf("%d\n", problem_two(jumps_two));

    free_input(input);
    return 0;
}
