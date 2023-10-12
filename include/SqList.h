#ifndef SQLIST_H
#define SQLIST_H
#include "mydef.h"


#define MAXSIZE 100

typedef struct SqList
{
	ElemType *elem;
	int len;
}SqList;

void InitList_Sq(SqList *L);
void DestroyList_Sq(SqList *L);

int ListEmpty_Sq(SqList L);
int ListLength_Sq(SqList L);
Status GetElem_Sq(SqList L, int pos, ElemType *e);
Status LocateElem_Sq(SqList L, ElemType e, int compare(ElemType, ElemType));

Status ListInsert_Sq(SqList *L, int loc, ElemType e);
Status ListDelete_Sq(SqList *L, int loc, ElemType* e);

Status Traverse_Sq(SqList L, Status visite(ElemType));

#endif

