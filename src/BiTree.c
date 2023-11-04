#include "BiTree.h"
#include <stdlib.h>
#include <stdio.h>
#include "LinkStack.h"

void CreateBiTree(BiTree *T)
{
    TElemType e;
    BTNode *p;
    printf("Please input an integer:\n");
    scanf("%d", &e);

    if(e > 0)
    {
        p = (BTNode*)malloc(sizeof(BTNode));
        p->data = e;
        *T = p;
        printf("Create node %d\n", e);
        CreateBiTree(&(p->lchild));
        CreateBiTree(&(p->rchild));
    }
    else
        *T = NULL;
} // BTNode**                    


void DestroyBiTree(BiTree *T)
{
    if(*T)
    {
        DestroyBiTree(&((*T)->lchild));
        DestroyBiTree(&((*T)->rchild));
        printf("Destroy node %d\n", (*T)->data);
        free(*T);
    }
}


void PrintBiTree(BiTree T, int layer)
{
    int i;
    if(T)
    {
        PrintBiTree(T->rchild, layer+1);
        for(i = 0; i < layer; i++)
            printf("  ");
        printf("%2d\n", T->data);
        PrintBiTree(T->lchild, layer+1);
    }
}

void PreOrder(BiTree T)
{
    if(T)
    {
        printf("visit %d\n", T->data);
        PreOrder(T->lchild);
        PreOrder(T->rchild);
    }
}

void InOrder(BiTree T)
{
    if(T)
    {
        InOrder(T->lchild);
        printf("visit %d\n", T->data);
        InOrder(T->rchild);
    }
}

void PostOrder(BiTree T)
{
    if(T)
    {
        PostOrder(T->lchild);
        PostOrder(T->rchild);
        printf("visit %d\n", T->data);
    }
}