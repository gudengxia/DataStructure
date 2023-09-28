#ifndef LINKLIST_H
#define LINKLIST_H
#include "mydef.h"

typedef struct LinkNode{
    ElemType e;
    struct LinkNode* next;
}LinkNode;

//typedef LinkNode * LinkList;
typedef struct LinkList{
    LinkNode *h;
    int len;
}LinkList;

/*LinkNode* Build_H();
LinkNode* Build_T();*/


void InitList_L(LinkList* L);

void Traverse_L(LinkList L);

void DestroyList_L(LinkList *L);// LinkNode **L

Status GetElem_L(LinkList L, int loc, LinkNode **e);

int ListLength_L(LinkList L);

Status ListInsert_L(LinkList L, int loc, ElemType e);

Status ListDelete_L(LinkList L, int loc, ElemType *e);

#endif

