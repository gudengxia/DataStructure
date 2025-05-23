#include "LinkQueue.h"
#include <stdlib.h>
Status InitQueue_L(LinkQueue *Q)
{
    QNode* p = (QNode*)malloc(sizeof(QNode));
    p->next = NULL;
    Q->front = Q->rear = p;
    return OK;
}
Status DestroyQueue_L(LinkQueue *Q)
{
    QNode* p = Q->front;
    QNode* q;
    while(p)
    {
        q = p;
        p = p->next;
        free(q);
    }
    Q->front = Q->rear = NULL;
    return OK;
}

int QueueLength_L(LinkQueue Q)
{
    int len = 0;
    QNode *p = Q.front;

    while(p != Q.rear)
    {
        p = p->next;
        len++;
    }
    return len;
}

Status EnQueue_L(LinkQueue* Q, QElemType e)
{
    QNode *p = (QNode*)malloc(sizeof(QNode));
    p->data = e;
    p->next = NULL;
    Q->rear->next = p;
    Q->rear = p;
    return OK;
}

Status DeQueue_L(LinkQueue* Q, QElemType* e)
{
    if(Q->rear == Q->front)
        return ERROR;
    QNode *p = Q->front->next;
    Q->front->next = p->next;
    if(Q->rear == p)
    {
        Q->rear = Q->front;
    }
    *e = p->data;
    free(p);
    return OK;
}

int QueueEmpty_L(LinkQueue Q)
{
    if(Q.front == Q.rear)
        return TRUE;
    else
        return FALSE;
}

Status GetHead_L(LinkQueue Q, QElemType *e)
{
    if(Q.front == Q.rear)
        return ERROR;
    *e = Q.front->next->data;
    return OK;
}