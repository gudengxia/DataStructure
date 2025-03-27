#ifndef LINKQUEUE_H
#define LINKQUEUE_H

#include "mydef.h"

typedef struct QNode{
    QElemType data;
    struct QNode *next;
}QNode;

typedef struct LinkQueue{
    QNode *front;
    QNode *rear;
}LinkQueue;

Status InitQueue_L(LinkQueue *Q);
Status DestroyQueue_L(LinkQueue *Q);

int QueueLength_L(LinkQueue Q);

Status EnQueue_L(LinkQueue* Q, QElemType e);
Status DeQueue_L(LinkQueue* Q, QElemType* e);

int QueueEmpty_L(LinkQueue Q);
Status GetHead_L(LinkQueue Q, QElemType *e);
#endif // LINKQUEUE_H

