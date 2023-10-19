#ifndef MYDEF_H
#define MYDEF_H

#define OK 1
#define ERROR 0
#define TRUE 1
#define FALSE 0
#define OVERFLOW -2

typedef int Status;
#ifdef MazeSolutionByStack
    typedef struct Location{
        int x;
        int y;
    }Location;

    typedef struct
    {
        //int ord;  //通道块在路径上的“序号”
        Location pos;  //通道块在迷宫中的“坐标位置”
        int direction;  //从此通道块走向下一通道块的“方向”
    }ElemType; 
#else
    #ifdef MazeSolutionByQueue
        typedef struct Location{
            int x;
            int y;
        }Location;
        typedef Location ElemType;
    #else
        typedef int ElemType;
    #endif
#endif 

Status visit(ElemType e);
int equal(ElemType e1, ElemType e2);

#endif 
