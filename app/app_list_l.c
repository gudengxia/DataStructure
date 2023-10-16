#include "LinkList.h"
#include <stdio.h>

int main()
{
    int i;
    ElemType e;
    LinkList L;

    InitList_L(&L);
    
    for(i = 0; i < 5; i++)
    {
        ListInsert_L(L, i+1, i*i);
    }
    
    ListInsert_L(L, 1, -1);
    ListInsert_L(L, 3, -2);
    printf("len_list = %d\n", ListLength_L(L));
    ListTraverse_L(L, visit);

    ListDelete_L(L, 3, &e);
    printf("Remove %d from the list.\n", e);
    ListDelete_L(L, 1, &e);
    printf("Remove %d from the list.\n", e);

    printf("len_list = %d\n", ListLength_L(L));
    
    ListTraverse_L(L, visit);
    DestroyList_L(&L);
}