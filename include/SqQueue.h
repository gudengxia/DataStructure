#ifndef SQ_QUEUE_H
#define SQ_QUEUE_H
#include "mydef.h"

#define MAXQUEUESIZE 10

typedef struct SqQueue{
    ElemType *base;
    int front;
    int rear;
} SqQueue;

void InitQueue_Sq(SqQueue* Q);
void DestroyQueue_Sq(SqQueue *Q);

int QueueLength_Sq(SqQueue Q);

Status EnQueue_Sq(SqQueue* Q, ElemType e);
Status DeQueue_Sq(SqQueue* Q, ElemType* e);

int QueueEmpty_Sq(SqQueue Q);
Status GetHead_Sq(SqQueue, ElemType *e);
#endif // QUEUE_SQ_H_INCLUDED

