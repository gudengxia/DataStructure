#ifndef LINKSTACK_H
#define LINKSTACK_H
#include "mydef.h"
typedef struct StackNode
{
    SElemType data;
    struct StackNode* next; 
}StackNode;

typedef StackNode *LinkStack;

void InitStack_L(LinkStack *S);
void DestroyStack_L(LinkStack *S);

Status Push_L(LinkStack *S, SElemType e);
Status Pop_L(LinkStack *S, SElemType *e);

Status GetTop_L(LinkStack S, SElemType *e);
int StackEmpty_L(LinkStack S); 
int StackLength_L(LinkStack S);
#endif