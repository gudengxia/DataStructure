#include "SqStack.h"
#include <stdlib.h>

Status InitStack_Sq(SqStack *S)
{
    S->base = (SElemType *)malloc(sizeof(SElemType) * MAXSTACKSIZE);
    S->top = S->base;
    S->stack_size = MAXSTACKSIZE;
    return OK;
}

Status DestroyStack_Sq(SqStack *S)
{
    if(S->base)
    {
        free(S->base);
        S->base = S->top = NULL;
        S->stack_size = 0;
    }
    return OK;
}


Status Push_Sq(SqStack *S, SElemType e)
{
    if(S->top - S->base == S->stack_size - 1)
        return ERROR;
    *(S->top) = e;
    S->top = S->top + 1;
    return OK;
}

Status Pop_Sq(SqStack *S, SElemType *e)
{
    if(S->top == S->base || S->stack_size <= 0)
        return ERROR;
    S->top = S->top - 1;
    *e = *(S->top);
    return OK;
}

int StackLength_Sq(SqStack S)
{
    return S.top - S.base;
}

Status GetTop_Sq(SqStack S, SElemType *e)
{
    if(S.top - S.base == 0)
        return ERROR;
    *e = *(S.top-1);
    return OK;
}

int StackEmpty_Sq(SqStack S)
{
    if(S.top == S.base)
        return 1;
    else
        return 0;
}