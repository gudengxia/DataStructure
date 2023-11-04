#ifndef BITREE_HPP
#define BITREE_HPP
#include "BiTree.h"
#include <iostream>
using namespace std;

typedef struct SState
{
    BTNode *p;
    bool flag; //Stay in the stack when Pop() if false; Pop out of the stack when Pop() if true
}SState;

class BinaryTree
{
private:
    BTNode* root;

public:
    BinaryTree();

    ~BinaryTree();

    void PreOrderTraverse();
    void InOrderTraverse();
    void PostOrderTraverse();
};

#endif
