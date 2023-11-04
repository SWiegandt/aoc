#include "btree.h"
#include <stdio.h>
#include <stdlib.h>

static BTreeNode* node(BTree* tree, BTreeNode* parent) {
    BTreeNode* node = malloc(sizeof(BTreeNode));
    node->left = node->right = node->item = NULL;
    node->tree = tree;
    node->parent = parent;

    return node;
}

BTree* btree(int (*cmp)(const void*, const void*)) {
    BTree* tree = malloc(sizeof(BTree));
    tree->root = node(tree, NULL);
    tree->cmp = cmp;

    return tree;
}

void tree_add(BTree* tree, void* item) {
    BTreeNode* current_node = tree->root;

    while (current_node->item != NULL) {
        if (tree->cmp(current_node->item, item) <= 0) {
            current_node = current_node->left;
        } else {
            current_node = current_node->right;
        }
    }

    current_node->item = item;
    current_node->left = node(tree, current_node);
    current_node->right = node(tree, current_node);
}

int tree_contains(BTree* tree, void* item) {
    BTreeNode* current_node = tree->root;

    while (current_node->item != NULL) {
        int cmp = tree->cmp(current_node->item, item);

        if (cmp < 0) {
            current_node = current_node->left;
        } else if (cmp > 0) {
            current_node = current_node->right;
        } else {
            return 1;
        }
    }

    return 0;
}

void free_btree(BTree* tree) {
    BTreeNode* node = tree->root;
    int freed_nodes = 0;

    while (node != NULL) {
        if (node->left != NULL) {
            BTreeNode* prev = node;
            node = prev->left;
            prev->left = NULL;
        } else if (node->right != NULL) {
            BTreeNode* prev = node;
            node = prev->right;
            prev->right = NULL;
        } else {
            BTreeNode* parent = node->parent;
            free(node->item);
            free(node);
            node = parent;
            freed_nodes++;
        }
    }

    free(tree);
}
