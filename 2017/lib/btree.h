#pragma once

typedef struct BTree BTree;
typedef struct BTreeNode BTreeNode;

struct BTree {
    BTreeNode* root;
    int (*cmp)(const void*, const void*);
};

struct BTreeNode {
    BTree* tree;
    BTreeNode* parent;
    BTreeNode* left;
    BTreeNode* right;
    void* item;
};

BTree* btree(int (*cmp)(const void*, const void*));
void tree_add(BTree* tree, void* item);
int tree_contains(BTree* tree, void* item);
void free_btree(BTree* tree);
