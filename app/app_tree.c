#include <stdio.h>
#include <stdlib.h>
#include "BiTree.h"
int main()
{
    BiTree T;

    CreateBiTree(&T);
    PrintBiTree(T, 1);
    DestroyBiTree(&T);
    return 0;
}