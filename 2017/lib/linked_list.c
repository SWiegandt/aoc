#include "linked_list.h"
#include <stdlib.h>
#include <string.h>

List* list() {
    List* list = malloc(sizeof(List));
    list->head = NULL;
    return list;
}

static Node* create_node(void* value) {
    Node* node = malloc(sizeof(Node));
    node->next = NULL;
    node->value = value;
    return node;
}

void push(List* list, void* value) {
    Node* node = create_node(value);
    node->next = list->head;
    list->head = node;
}

void* pop(List* list) {
    if (list->head == NULL) {
        return NULL;
    }

    Node* head = list->head;
    void* value = head->value;
    list->head = head->next;
    free(head);

    return value;
}

int contains(List* list, char* value) {
    Node* node = list->head;

    while (node != NULL) {
        if (strcmp(node->value, value) == 0) {
            return 1;
        }

        node = node->next;
    }

    return 0;
}

void free_list(List* list) {
    Node* current_node = list->head;

    while (current_node != NULL) {
        Node* next = current_node->next;
        free(current_node->value);
        free(current_node);
        current_node = next;
    }

    free(list);
}
