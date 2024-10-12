#pragma once

#include <stdio.h>
#include "linked_list.h"

#define BUCKETS 10000
#define KEY_SIZE 100

typedef struct {
    List* buckets[BUCKETS];
} HashMap;

typedef struct {
    void* key;
    void* value;
} KeyValue;

HashMap* hashmap();
void map_add(HashMap* map, void* key, void* value);
void* map_get(HashMap* map, const void* key, size_t key_length);
int map_has(HashMap* map, const void* key);
void free_hashmap(HashMap* map);
