#pragma once

#include <stdio.h>

typedef struct {
    char* input;
    char* next_token;
} FileInput;

FILE* file_input(int day);
FileInput* read_input(int day);
char* next_line(FileInput* input);
void free_input(FileInput* input);

#define loop_file(input, idx, line)                   \
    int idx = 0;                                      \
    for (char* line = next_line(input); line != NULL; \
         line = next_line(input), idx++)
