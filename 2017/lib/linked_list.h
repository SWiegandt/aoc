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
int contains(List* list, char* value);
void free_list(List* list);
