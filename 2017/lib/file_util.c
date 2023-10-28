#include "file_util.h"
#include <errno.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern int errno;

FILE* read_input(int day) {
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

char* string_input(int day) {
    FILE* fp = read_input(day);
    fseek(fp, 0, SEEK_END);
    long length = ftell(fp);
    rewind(fp);

    char* buffer = calloc(length, 1);
    char ch;
    size_t i = 0;

    if (buffer == NULL) {
        fclose(fp);
        fprintf(stderr, "Couldn't allocate memory: %s\n", strerror(errno));
        exit(1);
    }

    while ((ch = fgetc(fp)) != EOF) {
        buffer[i++] = ch;
    }

    // Trim trailing newlines
    while (buffer[--i] == '\n') {
        buffer[i] = '\0';
    }

    fclose(fp);
    return buffer;
}
