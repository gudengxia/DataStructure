#ifndef BITREE_H
#define BITREE_H
#include "mydef.h"

typedef struct BTNode
{
        TElemType data;
        struct BTNode *lchild, *rchild;
}BTNode;



typedef BTNode* BiTree;

#ifdef __cplusplus
extern "C" {
#endif
void CreateBiTree(BiTree *T); // BTNode**                                                  
void DestroyBiTree(BiTree *T);

void PreOrder(BiTree T);
void InOrder(BiTree T);
void PostOrder(BiTree T);

void PrintBiTree(BiTree T, int layer);

#ifdef __cplusplus
}
#endif

#endif