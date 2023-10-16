#ifndef LINKLIST_H
#define LINKLIST_H

#include "mydef.h"

typedef struct LinkNode
{
	ElemType data;
	struct LinkNode *next; 
}LinkNode, *LinkList;

void InitList_L(LinkList* L);
void DestroyList_L(LinkList* L);

void ClearList_L(LinkList* L);
int ListEmpty_L(LinkList L);
int ListLength_L(LinkList L);
Status GetElem_L(LinkList L, int i, ElemType *e);
LinkNode* LocateElem_L(LinkList L, ElemType e, Status compare(ElemType, ElemType));

Status ListInsert_L(LinkList L, int i, ElemType e);
Status ListDelete_L(LinkList L, int i, ElemType* e);

Status ListTraverse_L(LinkList L, Status visit(ElemType e));

#endif

