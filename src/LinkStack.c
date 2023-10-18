#include "LinkStack.h"
#include <stdlib.h>

void InitStack_L(LinkStack *S)
{
    *S = NULL;
}

void DestroyStack_L(LinkStack *S)
{
    StackNode *p = *S;
    StackNode *q;
    while(p)
    {
        q = p;
        p = p->next;
        free(q);
    }
    *S = NULL;
}

Status Push_L(LinkStack *S, ElemType e)
{
    StackNode *p = (StackNode*)malloc(sizeof(StackNode));
    p->data = e;
    p->next = *S;
    *S = p;
    return OK;
}

Status Pop_L(LinkStack *S, ElemType *e)
{
    StackNode *p = *S;
    if(p == NULL)
        return ERROR;
    *e = p->data;
     *S = p->next;
     free(p);
     return OK;
}

Status GetTop_L(LinkStack S, ElemType *e)
{
    if(!S)
        return ERROR;
    *e = S->data;
    return OK;
}

int StackEmpty_L(LinkStack S)
{
    if(!S)
        return 1;
    else
        return 0;
} 
int StackLength_L(LinkStack S)
{

}