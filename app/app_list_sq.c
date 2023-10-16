#include "SqList.h"
#include <stdio.h>

int main()
{
    int i;
    ElemType e;
    SqList L;

    InitList_Sq(&L);
    for(i = 0; i < 5; i++)
    {
        ListInsert_Sq(&L, i+1, i*i);
    }
    ListInsert_Sq(&L, 1, -1);
    ListInsert_Sq(&L, 3, -2);
    printf("len_list = %d\n", ListLength_Sq(L));
    ListTraverse_Sq(L, visit);

    ListDelete_Sq(&L, 3, &e);
    printf("Remove %d from the list.\n", e);
    ListDelete_Sq(&L, 1, &e);
    printf("Remove %d from the list.\n", e);

    printf("len_list = %d\n", ListLength_Sq(L));
    ListTraverse_Sq(L, visit);
    DestroyList_Sq(&L);
}