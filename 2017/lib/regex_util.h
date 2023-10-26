#pragma once

#include <regex.h>

int re_compile(regex_t* preg, const char* pattern, int flags);
char* re_group(const char* str, regmatch_t pmatch[], size_t group);
void re_next(char** str, regmatch_t pmatch[]);
