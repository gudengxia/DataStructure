#ifndef SQ_QUEUE_H
#define SQ_QUEUE_H
#include "mydef.h"

#define MAXQUEUESIZE 10

typedef struct SqQueue{
    QElemType *base;
    int front;
    int rear;
} SqQueue;

Status InitQueue_Sq(SqQueue* Q);
Status DestroyQueue_Sq(SqQueue *Q);

int QueueLength_Sq(SqQueue Q);

Status EnQueue_Sq(SqQueue* Q, QElemType e);
Status DeQueue_Sq(SqQueue* Q, QElemType* e);

int QueueEmpty_Sq(SqQueue Q);
Status GetHead_Sq(SqQueue, QElemType *e);
#endif // QUEUE_SQ_H_INCLUDED

