#include <stdlib.h>
#include <string.h>
#include "file_util.h"
#include "linked_list.h"

void noop(char* word) {}

int char_cmp(const void* c1, const void* c2) {
    return *(char*)c1 - *(char*)c2;
}

void sort(char* word) {
    qsort(word, strlen(word), 1, char_cmp);
}

int is_valid(char* passphrase, void (*word_modifier)(char*)) {
    List* words = list();
    int word_length = 0;

    for (int i = 0; i <= strlen(passphrase); i++) {
        if (passphrase[i] == ' ' || passphrase[i] == '\0') {
            char* word = malloc(word_length + 1);
            snprintf(word, word_length + 1, "%s", passphrase + i - word_length);
            word_modifier(word);

            if (!contains(words, word)) {
                push(words, word);
            } else {
                free_list(words);
                return 0;
            }
            word_length = 0;
        } else {
            word_length++;
        }
    }

    free_list(words);
    return 1;
}

int problem_one() {
    FileInput* input = read_input(4);
    int valid = 0;

    loop(input, passphase) {
        valid += is_valid(passphase, noop);
    }

    return valid;
}

int problem_two() {
    FileInput* input = read_input(4);
    int valid = 0;

    loop(input, passphase) {
        valid += is_valid(passphase, sort);
    }

    return valid;
}
int main() {
    printf("%d\n", problem_one());
    printf("%d\n", problem_two());

    return 0;
}
