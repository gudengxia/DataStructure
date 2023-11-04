#ifndef LINKSTACK_H
#define LINKSTACK_H
#include "mydef.h"
#include <cstdlib>
template <class T> struct StackNode
{
    T data;
    struct StackNode* next;
};

template <class T> class LinkStack
{
private:
    StackNode<T> *h;

public:
    LinkStack()
    {
        h = NULL;
    }

    ~LinkStack()
    {
        StackNode<T> *p = h;
        while(p)
        {
            h = p->next;
            delete p;
            p = h;
        }
        h = NULL;
    }

    Status Push(T e)
    {
        StackNode<T> *p = new StackNode<T>;
        p->data = e;
        p->next = h;
        h = p;
        return OK;
    }

    Status Pop(T *e)
    {
        if(h == NULL)
            return ERROR;
        StackNode<T> *p = h;
        *e = p->data;
        h = p->next;
        delete(p);
        return OK;
    }

    Status GetTop(T *e)
    {
        if(h == NULL)
            return ERROR;
        *e = h->data;
        return OK;
    }

    bool StackEmpty()
    {
        if(h == NULL)
            return true;
        else
            return false;
    }

    int StackLength()
    {
        int i = 0;
        StackNode<T>* p = h;
        while(p)
        {
            i++;
            p = p->next;
        }

        return i;
    }
};

#endif
