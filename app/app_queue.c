#ifndef LinkType
    #define Function(fname) fname##_##Sq
    #include "SqQueue.h"
#else
   #define Function(fname) fname##_##L
    #include "LinkQueue.h"
#endif

#include <stdio.h>

int main()
{
    int i;
    ElemType e;

    #ifdef LinkType
        LinkQueue Q;
    #else
        SqQueue Q;
    #endif
    Function(InitQueue)(&Q);
    
    for(i = 0; i < 10; i++)
    {
        if(Function(EnQueue)(&Q, i*i))
        {
            printf("Element %d enters into the queue.\n", i*i);
        }
        else
        {
            printf("Fail to insert %d into the queue.\n", i*i);
        }
    }

    for(i = 0; i < 11; i++)
    {
        if(Function(DeQueue)(&Q, &e))
        {
            printf("Delete %d from the queue.\n", e);
        }
        else
        {
            printf("Fail to delete an element from the queue.\n");
        }
    }

    Function(DestroyQueue)(&Q);
    return 0;
}