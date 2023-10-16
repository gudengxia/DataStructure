#include "SqList.h"
#include <stdlib.h>
#include <stdio.h>

void InitList_Sq(SqList *L)
{
	L->elem = (ElemType*)malloc(sizeof(ElemType) * MAXSIZE);
	L->len = 0;
}

void DestroyList_Sq(SqList *L)
{
	free(L->elem);
}

int ListEmpty_Sq(SqList L)
{
	if(L.len == 0)
		return TRUE;
	else
		return FALSE;
}

int ListLength_Sq(SqList L)
{
	return L.len;
}

Status GetElem_Sq(SqList L, int pos, ElemType *e)
{
	if(pos < 1 || pos > L.len)
		return ERROR;
	*e = L.elem[pos-1];
	return OK;
}

Status LocateElem_Sq(SqList L, ElemType e, Status compare(ElemType, ElemType))
{
	int i;
	for(i = 0; i < L.len; i++)
		if(compare(L.elem[i], e))
			return i+1;
	return 0;
}

Status ListInsert_Sq(SqList *L, int loc, ElemType e)
{
	if(loc < 1 || loc > L->len + 1)
		return ERROR;
	if(L->len >= MAXSIZE)
		return OVERFLOW;
	int i;
	/*for(i = L->len-1; i >= loc-1; i--)
		L->elem[i+1] = L->elem[i];*/
    for(i = L->len; i >= loc; i--)
		L->elem[i] = L->elem[i-1];
	L->elem[loc-1] = e;
	L->len ++;
	return OK;
}

Status ListDelete_Sq(SqList *L, int loc, ElemType* e)
{
    int i;
    if(loc < 1 || loc > L->len)
        return ERROR;
    *e = L->elem[loc-1];
    for(i = loc; i <= L->len-1; i++)
    {
        L->elem[i-1] = L->elem[i];
    }
    L->len--;
    return OK;
}

Status ListTraverse_Sq(SqList L, Status visite(ElemType))
{
	int i;
	//printf("There are %d elements:\n", L.len);
	for(i = 0; i < L.len; i++)
		if(!visite(L.elem[i]))
            return ERROR;
	printf("\n");
	return OK;
}

