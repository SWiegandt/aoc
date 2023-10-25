#pragma once

#define REGEX(preg, pattern, flags)                                            \
    regex_t preg;                                                              \
    int err = regcomp(&preg, pattern, 0);                                      \
    if (err != 0) {                                                            \
        int err_length = regerror(err, &re, NULL, 0);                          \
        char *err_message = malloc(err_length);                                \
        regerror(err, &re, err_message, err_length);                           \
        fprintf(stderr, "Error compiling pattern: %s", err_message);           \
        free(err_message);                                                     \
        exit(1);                                                               \
    }
