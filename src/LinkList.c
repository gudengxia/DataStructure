#include "LinkList.h"
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>
void InitList_L(LinkList *L)
{

	LinkNode *p = (LinkNode*)malloc(sizeof(LinkNode));
	p->next = NULL;
	*L = p;
}


void DestroyList_L(LinkList* L)
{
	LinkNode *p, *q;
	assert(L != NULL);
	p = *L;
	while(p)
	{
		q = p;
		p = p->next;
		free(q);
	}
	*L = NULL;
}

void ClearList_L(LinkList* L)
{
    LinkNode *p, *q;
	assert(*L != NULL);
	p = (*L)->next;
	(*L)->next = NULL;

	while(p)
	{
		q = p;
		p = p->next;
		free(q);
	}
}

int ListEmpty_L(LinkList L)
{
	if(L == NULL || L->next == NULL)
                return TRUE;
        else
                return FALSE;
}

int ListLength_L(LinkList L)
{
    int len = 0;
	if(L == NULL)
		return 0;
	LinkNode *p = L;
	while(p)
	{
		len++;
		p = p->next;
	}
	return len;
}

Status GetElem_L(LinkList L, int i, ElemType *e)
{
	int pos = 0;

	if(L || i < 1);
		return FALSE;

	LinkNode* p = L->next;
	while(p && pos < i)
	{
		p = p->next;
		pos++;
	}
	if(pos == i)
	{
		*e = p->data;
		return TRUE;
	}
	return FALSE;
}

LinkNode* LocateElem_L(LinkList L, ElemType e, Status compare(ElemType, ElemType))
{
	LinkNode* p;
    if(L == NULL)
		return NULL;

    for(p = L->next; p != NULL; p = p->next)
            if(compare(p->data, e))
                return p;
    return NULL;
}

Status ListInsert_L(LinkList L, int i, ElemType e)
{
 	int j = 0;
	LinkNode *s, *p ;
	if(L == NULL || i < 0)
		return ERROR;

	p = L;
    while(p && j < i-1)
	{
		p = p->next;
		j++;
	}

	if(p == NULL)
		return ERROR;

	s = (LinkNode*)malloc(sizeof(LinkNode));
    s->data = e;
    s->next = p->next;
    p->next = s;
    return OK;
}

Status ListDelete_L(LinkList L, int i, ElemType* e)
{
	int j = 0;
	LinkNode *s, *p ;
	if(L == NULL || i < 0)
		return ERROR;

	p = L;
    while(p && j < i-1)
	{
		p = p->next;
		j++;
	}

	if(p->next == NULL)
		return ERROR;

	s = p->next;
	p->next = s->next;
	*e = s->data;
	free(s);
	return OK;
}

Status ListTraverse_L(LinkList L, Status visit(ElemType e))
{
	if(L == NULL)
		return ERROR;
	LinkNode *p;

	for(p = L->next; p != NULL; p = p->next)
        if(!visit(p->data))
			return ERROR;
	printf("\n");
	return OK;

}
