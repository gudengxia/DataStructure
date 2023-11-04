#ifndef MYDEF_H
#define MYDEF_H

#define OK 1
#define ERROR 0
#define TRUE 1
#define FALSE 0
#define OVERFLOW -2

typedef int Status;
typedef int ElemType;
typedef int TElemType;

typedef struct Location{
    int x;
    int y;
}Location;

#ifdef MazeSolutionByStack
    typedef struct
    {
        //int ord;  //通道块在路径上的“序号”
        Location pos;  //通道块在迷宫中的“坐标位置”
        int direction;  //从此通道块走向下一通道块的“方向”
    }SElemType; 
    typedef int QElemType;
#else
    #ifdef MazeSolutionByQueue
        typedef Location SElemType;
        typedef Location QElemType;
#else
    typedef int SElemType; 
    typedef int QElemType;
#endif
#endif




Status visit(ElemType e);
int equal(ElemType e1, ElemType e2);

#endif 
