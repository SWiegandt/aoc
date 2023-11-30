#include "hashmap.h"
#include <stdlib.h>
#include <string.h>
#include "linked_list.h"

HashMap* hashmap() {
    HashMap* map = malloc(sizeof(HashMap));

    for (int i = 0; i < BUCKETS; i++) {
        map->buckets[i] = list();
    }

    return map;
}

static unsigned long hash(unsigned char* str, size_t len) {
    unsigned long hash = 5381;
    int c = *str++;

    for (int i = 0; c && i < len; i++) {
        hash = ((hash << 5) + hash) + c; /* hash * 33 + c */
        c = *str++;
    }

    return hash;
}

static List* get_bucket(HashMap* map, const void* key) {
    unsigned long hash_key = hash((unsigned char*)key, KEY_SIZE);
    return map->buckets[hash_key % BUCKETS];
}

void map_add(HashMap* map, void* key, void* value) {
    List* bucket = get_bucket(map, key);
    KeyValue* key_value = malloc(sizeof(KeyValue));
    key_value->key = key;
    key_value->value = value;
    push(bucket, key_value);
}

void* map_get(HashMap* map, const void* key, size_t key_length) {
    List* bucket = get_bucket(map, key);
    Node* node = bucket->head;

    if (key_length == 0 || key_length > KEY_SIZE) {
        key_length = KEY_SIZE;
    }

    if (bucket->length == 1) {
        return ((KeyValue*)node->value)->value;
    }

    while (node != NULL &&
           memcmp(((KeyValue*)node->value)->key, key, key_length)) {
        node = node->next;
    }

    if (node == NULL) {
        return NULL;
    }

    return ((KeyValue*)node->value)->value;
}

int map_has(HashMap* map, const void* key) {
    return get_bucket(map, key)->length > 0;
}

void free_hashmap(HashMap* map) {
    for (int i = 0; i < BUCKETS; i++) {
        Node* node = map->buckets[i]->head;

        while (node != NULL) {
            Node* next = node->next;
            free(((KeyValue*)(node->value))->key);
            free(((KeyValue*)(node->value))->value);
            free(node->value);
            free(node);
            node = next;
        }

        free(map->buckets[i]);
    }

    free(map);
}
