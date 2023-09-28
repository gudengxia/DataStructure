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

void Traverse_Sq(SqList L)
{
	int i;
	printf("There are %d elements:\n", L.len);
	printf("[");
	for(i = 0; i < L.len; i++)
	{
		printf("%d,", L.elem[i]);
	}
	printf("]\n");
}

