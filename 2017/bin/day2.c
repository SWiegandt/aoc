#include "file_util.h"
#include "regex_util.h"
#include <regex.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    REGEX(re, "[[:digit:]]+", 0);
    char *input = string_input("2");
    char *line = strtok(input, "\n");
    regfree(&re);
    return 0;
}
