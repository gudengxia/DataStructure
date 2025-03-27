#ifndef SQSTACK_H
#define SQSTACK_H
#include "mydef.h"
#define MAXSTACKSIZE 100

typedef struct SqStack{
    SElemType *base;
    SElemType *top;
    int stack_size;
}SqStack;


Status InitStack_Sq(SqStack *S);
Status DestroyStack_Sq(SqStack *S);

Status Push_Sq(SqStack *S, SElemType e);
Status Pop_Sq(SqStack *S, SElemType *e);

Status GetTop_Sq(SqStack S, SElemType *e);
int StackEmpty_Sq(SqStack S); 
int StackLength_Sq(SqStack S);

#endif // SQSTACK_H

