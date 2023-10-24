#include "file_util.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    char *content = string_input("1");
    printf("file content: \n%s", content);
    free(content);

    return 0;
}
