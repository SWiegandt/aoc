#pragma once

typedef struct node {
    struct node* next;
    void* value;
} Node;

typedef struct {
    Node* head;
} List;

List* list();
void push(List* list, void* value);
void* pop(List* list);
int contains(List* list, const char* value);
void free_list(List* list);

#define loop_list(list, idx, node) \
    int idx = 0;                   \
    for (Node* node = list->head; node != NULL; node = node->next, idx++)
