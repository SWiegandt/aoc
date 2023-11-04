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
int list_contains_str(List* list, char* value);
int list_contains_int(List* list, int* value);
void free_list(List* list);
