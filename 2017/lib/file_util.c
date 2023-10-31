#include "file_util.h"
#include <errno.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern int errno;

FILE* file_input(int day) {
    char filename[16];
    snprintf(filename, 16, "input/%d.txt", day);
    FILE* fp = fopen(filename, "r");

    if (fp == NULL) {
        fprintf(stderr, "Couldn't open file %s: %s\n", filename,
                strerror(errno));
        exit(1);
    }

    return fp;
}

FileInput* read_input(int day) {
    FILE* fp = file_input(day);
    fseek(fp, 0, SEEK_END);
    long length = ftell(fp);
    rewind(fp);

    FileInput* finput = malloc(sizeof(FileInput));
    finput->input = finput->next_token = calloc(length, 1);
    char ch;
    size_t i = 0;

    if (finput->input == NULL) {
        fclose(fp);
        fprintf(stderr, "Couldn't allocate memory: %s\n", strerror(errno));
        exit(1);
    }

    while ((ch = fgetc(fp)) != EOF) {
        finput->input[i++] = ch;
    }

    // Trim trailing newlines
    while (finput->input[--i] == '\n') {
        finput->input[i] = '\0';
    }

    fclose(fp);
    return finput;
}

char* next_line(FileInput* input) {
    char* line = strtok(input->next_token, "\n");
    input->next_token = NULL;

    return line;
}

void free_input(FileInput* input) {
    free(input->input);
    free(input);
}
