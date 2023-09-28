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
Status ListInsert_Sq(SqList *L, int loc, ElemType e);
Status ListDelete_Sq(SqList *L, int loc, ElemType* e);
void Traverse_Sq(SqList L);

#endif

