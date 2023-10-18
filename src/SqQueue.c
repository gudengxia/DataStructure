#include "SqQueue.h"
#include <stdlib.h>

void InitQueue_Sq(SqQueue* Q)
{
    Q->base = (ElemType*) malloc(sizeof(ElemType) * MAXQUEUESIZE);
    Q->rear = Q->front = 0;
}

void DestroyQueue_Sq(SqQueue *Q)
{
    if(Q->base)
        free(Q->base);
    Q->base = NULL;
    Q->front = Q->rear = -1;
}

int QueueLength_Sq(SqQueue Q)
{
    return (Q.rear - Q.front + MAXQUEUESIZE) % MAXQUEUESIZE;
}

Status EnQueue_Sq(SqQueue* Q, ElemType e)
{
    if((Q->rear + 1) % MAXQUEUESIZE == Q->front)
        return ERROR;
    Q->base[Q->rear] = e;
    Q->rear = (Q->rear + 1) % MAXQUEUESIZE;
    return OK;

}

Status DeQueue_Sq(SqQueue* Q, ElemType* e)
{
    if(Q->front == Q->rear)
        return ERROR;
    *e = Q->base[Q->front];
    Q->front = (Q->front + 1) % MAXQUEUESIZE;
    return OK;
}

int QueueEmpty_Sq(SqQueue Q)
{
    if(Q.front == Q.rear)
        return TRUE;
    else
        return FALSE;
}

Status GetHead_Sq(SqQueue Q, ElemType *e)
{
    if(Q.front == Q.rear)
        return ERROR;
    *e = Q.base[Q.front];
    return OK;
}