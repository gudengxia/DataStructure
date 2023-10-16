#include "mydef.h"
#include <stdio.h>

Status visit(ElemType e)
{
    printf("%d ", e);
}
int equal(ElemType e1, ElemType e2)
{
    if(e1 == e2)
        return TRUE;
    else
        return FALSE;
}