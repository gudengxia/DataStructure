#ifndef MYDEF_H
#define MYDEF_H

#define OK 1
#define ERROR 0
#define TRUE 1
#define FALSE 0
#define OVERFLOW -2

typedef int Status;
#ifdef Maze
    typedef struct ElemType{
        int x;
        int y;
    }ElemType;
#else
    typedef int ElemType;
#endif 

Status visit(ElemType e);
int equal(ElemType e1, ElemType e2);

#endif 
