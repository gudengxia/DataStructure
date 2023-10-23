
#ifdef LinkType
    #define Function(fname) fname##_##L
    #include "LinkStack.h"
#else
    #define Function(fname) fname##_##Sq
    #include "SqStack.h"
#endif

#include <stdio.h>
int main()
{
    ElemType e;
    int i;
    #ifdef LinkType
        LinkStack S;
    #else
        SqStack S;
    #endif
    Function(InitStack)(&S);
    
    for(i = 0; i < 5; i++)
    {
        Function(Push)(&S, i*i);
        printf("Push %d into the stack.\n", i*i);
    }

    for(i = 0; i < 5; i++)
    {
        Function(Pop)(&S, &e);
        printf("Pop %d from the stack.\n", e);
    }

    if(Function(StackEmpty)(S))
    {
        printf("Now the stack is empty.\n");
    }
    else
    {
        printf("Error happens.\n");
    }
    Function(DestroyStack)(&S);
    return 0;
}
