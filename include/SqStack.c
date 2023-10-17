#include "SqStack.h"
#include <stdlib.h>

void InitStack(SqStack *S, int max_size)
{
    S->base = (ElemType *)malloc(sizeof(ElemType) * max_size);
    S->top = S->base;
    S->stack_size = max_size;
}

void DestroyStack(SqStack *S)
{
    if(S->base)
    {
        free(S->base);
        S->base = S->top = NULL;
        S->stack_size = 0;
    }
}


Status Push(SqStack *S, ElemType e)
{
    if(S->top - S->base == S->stack_size - 1)
        return ERROR;
    *(S->top) = e;
    S->top = S->top + 1;
    return OK;
}

Status Pop(SqStack *S, ElemType *e)
{
    if(S->top == S->base || S->stack_size <= 0)
        return ERROR;
    S->top = S->top - 1;
    *e = *(S->top);
    return OK;
}

int StackLen(SqStack S)
{
    return S.top - S.base;
}

