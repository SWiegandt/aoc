#include <limits.h>
#include <stdlib.h>
#include <string.h>
#include "file_util.h"
#include "linked_list.h"

#define BANKS 16

int main() {
    int banks[BANKS] = {0};
    FileInput* input = read_input(6);
    int max_pos = 0;
    int max_val = INT_MIN;
    int rounds = 0;
    List* states = list();

    loop_file(input, idx, line) {
        banks[idx] = atoi(line);

        if (banks[idx] > max_val) {
            max_pos = idx;
            max_val = banks[idx];
        }
    }

    while (1) {
        char* state = calloc(128, 1);

        for (int i = 0; i < BANKS; i++) {
            char* st = calloc(128, 1);
            strcpy(st, state);
            sprintf(state, "%s,%d", st, banks[i]);
            free(st);
        }

        loop_list(states, list_idx, node) {
            if (strcmp(node->value, state) == 0) {
                printf("cycle length: %d\n", list_idx + 1);
                free(state);
                goto done;
            }
        }

        push(states, state);

        int blocks = max_val;
        max_val = 0;
        banks[max_pos] = 0;

        for (int i = (max_pos + 1) % BANKS; blocks > 0;
             i = (i + 1) % BANKS, blocks--) {
            banks[i]++;
        }

        for (int i = 0; i < BANKS; i++) {
            if (banks[i] > max_val) {
                max_pos = i;
                max_val = banks[i];
            }
        }

        rounds++;
    }

done:
    printf("rounds: %d\n", rounds);

    free_input(input);
    free_list(states);
    return 0;
}
