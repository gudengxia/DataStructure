#ifndef SQSTACK_H
#define SQSTACK_H
#include "mydef.h"

typedef struct SqStack{
    ElemType *base;
    ElemType *top;
    int stack_size;

}SqStack;


void InitStack(SqStack *S, int max_size);
void DestroyStack(SqStack *S);

Status Push(SqStack *S, ElemType e);
Status Pop(SqStack *S, ElemType *e);

int StackLen(SqStack S);

#endif // SQSTACK_H

