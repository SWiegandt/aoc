#pragma once

#include <stdio.h>

typedef struct {
    char* input;
    char* next_token;
} FileInput;

FILE* file_input(int day);
FileInput* read_input(int day);
char* next_line(FileInput* input);
void cleanup(FileInput* input);

#define loop(input, line) \
    char* line;           \
    while ((line = next_line(input)) != NULL)
