#include <regex.h>
#include <stdio.h>
#include <stdlib.h>

static void re_error(const regex_t* re, const char* description, int err) {
    char err_message[100];
    regerror(err, re, err_message, 100);
    fprintf(stderr, "%s: %s", description, err_message);
}

int re_compile(regex_t* re, const char* pattern, int flags) {
    int err = regcomp(re, pattern, flags);

    if (err != 0) {
        re_error(re, "Error compiling pattern", err);
        return 1;
    }

    return 0;
}

char* re_group(const char* str, regmatch_t pmatch[], size_t group) {
    int len = pmatch[group].rm_eo - pmatch[group].rm_so;
    char* match = calloc(len + 1, 1);
    snprintf(match, len + 1, "%s", str + pmatch[group].rm_so);

    return match;
}

void re_next(char** str, regmatch_t pmatch[]) {
    *str += pmatch[0].rm_eo;
}
