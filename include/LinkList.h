#ifndef LINKLIST_H
#define LINKLIST_H

#include "mydef.h"

typedef struct LinkNode
{
	ElemType data;
	struct LinkNode *next; 
}LinkNode, *LinkList;

Status InitList_L(LinkList* L);
Status DestroyList_L(LinkList* L);

Status ClearList_L(LinkList* L);
int ListEmpty_L(LinkList L);
int ListLength_L(LinkList L);
Status GetElem_L(LinkList L, int i, ElemType *e);
LinkNode* LocateElem_L(LinkList L, ElemType e, Status compare(ElemType, ElemType));

Status ListInsert_L(LinkList L, int i, ElemType e);
Status ListDelete_L(LinkList L, int i, ElemType* e);

Status ListTraverse_L(LinkList L, Status visit(ElemType e));

#endif

