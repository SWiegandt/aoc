#include "file_util.h"
#include <errno.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern int errno;

FILE *read_input(const char *day) {
    int size = snprintf(NULL, 0, "../input/%s.txt", day);
    char *filename = malloc(size + 1);
    sprintf(filename, "../input/%s.txt", day);
    FILE *fp = fopen(filename, "r");

    if (fp == NULL) {
        fprintf(stderr, "Couldn't open file %s: %s\n", filename,
                strerror(errno));
        free(filename);
        exit(errno);
    }

    free(filename);
    return fp;
}

char *string_input(const char *day) {
    FILE *fp = read_input(day);
    fseek(fp, 0, SEEK_END);
    long length = ftell(fp);
    rewind(fp);

    char *buffer = calloc(length, sizeof(char));
    char ch;
    size_t i = 0;

    if (buffer == NULL) {
        fprintf(stderr, "Couldn't allocate memory: %s\n", strerror(errno));
        exit(errno);
    }

    while ((ch = fgetc(fp)) != EOF) {
        if (ch == '\n') {
            break;
        }

        buffer[i++] = ch;
    }

    fclose(fp);
    return buffer;
}

void print_input(const char *day) {
    FILE *fp = read_input(day);
    char ch;

    while ((ch = fgetc(fp)) != EOF) {
        putchar(ch);
    }

    fclose(fp);
}
