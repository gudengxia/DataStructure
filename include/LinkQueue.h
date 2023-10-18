#ifndef LINKQUEUE_H
#define LINKQUEUE_H

#include "mydef.h"

typedef struct QNode{
    ElemType data;
    struct QNode *next;
}QNode;

typedef struct LinkQueue{
    QNode *front;
    QNode *rear;
}LinkQueue;

void InitQueue_L(LinkQueue *Q);
void DestroyQueue_L(LinkQueue *Q);

int QueueLength_L(LinkQueue Q);

Status EnQueue_L(LinkQueue* Q, ElemType e);
Status DeQueue_L(LinkQueue* Q, ElemType* e);

int QueueEmpty_L(LinkQueue Q);
Status GetHead_L(LinkQueue Q, ElemType *e);
#endif // LINKQUEUE_H

