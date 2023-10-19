#ifndef SQSTACK_H
#define SQSTACK_H
#include "mydef.h"
#define MAXSTACKSIZE 100
typedef struct SqStack{
    ElemType *base;
    ElemType *top;
    int stack_size;

}SqStack;


void InitStack_Sq(SqStack *S);
void DestroyStack_Sq(SqStack *S);

Status Push_Sq(SqStack *S, ElemType e);
Status Pop_Sq(SqStack *S, ElemType *e);

Status GetTop_Sq(SqStack S, ElemType *e);
int StackEmpty_Sq(SqStack S); 
int StackLength_Sq(SqStack S);

#endif // SQSTACK_H

