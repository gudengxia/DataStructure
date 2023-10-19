#include "mydef.h"
#include "LinkStack.h"
#include "LinkQueue.h"
#include <stdio.h>
int Maze[10][10] =    //迷宫地图为全局变量
{
   //0,1,2,3,4,5,6,7,8,9
    {0,0,0,0,0,0,0,0,0,0}, //0
    {0,1,1,0,1,1,1,0,1,0}, //1
    {0,1,1,0,1,1,1,0,1,0}, //2
    {0,1,1,1,1,0,0,1,1,0}, //3
    {0,1,0,0,0,1,1,1,1,0}, //4
    {0,1,1,1,0,1,1,1,1,0}, //5
    {0,1,0,1,1,1,0,1,1,0}, //6
    {0,1,0,0,0,1,0,0,1,0}, //7
    {0,0,1,1,1,1,1,1,1,0}, //8
    {0,0,0,0,0,0,0,0,0,0}  //9
};

void PrintMaze( )
{
    printf("\n 迷宫的地图为：\n\n");
    int i,j;
    for(i = 0;i<10;i++){
        for(j = 0;j<10;j++)
            printf(" %3d",Maze[i][j]);
        printf("\n");
    }
}

/*Status FootPrint(Location pos,int curstep)
{
    Maze[pos.x][pos.y] = curstep;
    return OK;
}*/


Status Pass(Location pos)
{
    if(Maze[pos.x][pos.y] == 1)
        return TRUE;

    return FALSE;
}

Location NextPos(Location pos,int i)
{
    switch(i)
    {
    case 1:
        pos.y++;  //东
            break;
    case 2:
        pos.x++;  //南
            break;
    case 3:
        pos.y--;  //西
            break;
    case 4:
        pos.x--; //北
            break;
    }//switch
    return pos;
}

void MakePrint(Location pos)
{
    //printf("(%d,%d)走不通\n",pos.x,pos.y);
    Maze[pos.x][pos.y] = -1;  //将走不通的块替换为墙壁
}
#ifdef MazeSolutionByStack
void MazeSolution(Location start, Location end)
{
    LinkStack S;
    ElemType curpos, nextpos;
    InitStack_L(&S);
    curpos.pos = start;
    curpos.direction = 1;
    Push_L(&S, curpos);
    MakePrint(curpos.pos);
    while(!StackEmpty_L(S))
    {
        GetTop_L(S, &curpos);

        if(curpos.pos.x == end.x && curpos.pos.y == end.y)
        {
            printf("Reach then end.\n");
            break;
        }

        nextpos.pos = NextPos(curpos.pos, curpos.direction);

        if(Pass(nextpos.pos))
        {
            nextpos.direction = 1;
            Push_L(&S, nextpos);
            printf("Go to (%d, %d).\n", nextpos.pos.x, nextpos.pos.y);
        }
        else
        {
            Pop_L(&S, &curpos);
            if(curpos.direction < 4)
            {
                curpos.direction++;
                Push_L(&S, curpos);
                MakePrint(curpos.pos);
                printf("Change to direction %d at (%d, %d)\n", curpos.direction, curpos.pos.x, curpos.pos.y);
            }
            else
            {
                printf("Backward from (%d, %d).\n", curpos.pos.x, curpos.pos.y);
            }
        }
    }

    if(StackEmpty_L(S))
    {
        printf("Sorry, there is no path.\n");
    }
    else
    {
        printf("End<--\n");
        while(!StackEmpty_L(S))
        {
            Pop_L(&S, &curpos);
            printf("(%d, %d)<--", curpos.pos.x, curpos.pos.y);
        }
    }
    printf("Start\n");
    DestroyStack_L(&S);
}
#endif 

#ifdef MazeSolutionByQueue
void MazeSolution(Location start, Location end)
{
    int i;
    LinkQueue Q;
    ElemType curpos, nextpos;

    curpos = start;
    InitQueue_L(&Q);
    EnQueue_L(&Q, curpos);

    while(!QueueEmpty_L(Q))
    {
        GetHead_L(Q, &curpos);
        if(curpos.x == end.x && curpos.y == end.y)
        {
            break;
        }
        
        DeQueue_L(&Q, &curpos);
       
        for(i = 1; i <= 4; i++)
        {
            nextpos = NextPos(curpos, i);
            if(Pass(nextpos))
            {
                EnQueue_L(&Q, nextpos);
                Maze[nextpos.x][nextpos.y] = Maze[curpos.x][curpos.y] + 1;
            }
        }

    }
    if(QueueEmpty_L(Q))
    {
        printf("Sorry, there is no path.\n");
    }
    else
    {
        Maze[start.x][start.y] = 1;
        PrintMaze();
        LinkStack S;
        
        InitStack_L(&S);
        Push_L(&S, end);
        while(1)
        {
            GetTop_L(S, &curpos);
            if(curpos.x == start.x && curpos.y == start.y)
                break;
            for(i = 1; i <=4; i++)
            {
                nextpos = NextPos(curpos, i);
                if(Maze[nextpos.x][nextpos.y] == Maze[curpos.x][curpos.y] - 1)
                    Push_L(&S, nextpos);
            }
        }

        printf("Start-->");
        while(!StackEmpty_L(S))
        {
            Pop_L(&S, &curpos);
            printf("(%d, %d)-->", curpos.x, curpos.y);
        }
        printf("End\n");
        DestroyStack_L(&S);
    }
    DestroyQueue_L(&Q);
}
#endif 

/*******************************主函数部分**************************************/

int main()
{
    PrintMaze();
    Location start,end;
    start.x = 1;
    start.y = 1;
    end.x = 8;
    end.y = 8;
   
    MazeSolution(start,end);
    
    //PrintMaze();
    return 0;
}

/*Status MazePath(Location start, Location end)
{
    SqStack S;
    Location curpos;
    ElemType e;
    int curstep;

    InitStack_L(&S);
    curpos = start; //设定“当前位置”为“入口位置”
    curstep = 1; //探索第一步
    printf("\n起点：(%d,%d)",start.x,start.y);
    do{
        if(Pass(curpos)){  //当前位置可以通过，即使未曾走到过的通道块
            FootPrint(curpos,curstep);  //留下足迹
            printf("  ->(%3d,%3d)\n",curpos.x,curpos.y);
            e.ord = curstep;
            e.pos = curpos;
            e.direction = 1;
            Push_L(&S,e);  //加入路径

            if(curpos.x == end.x && curpos.y == end.y){  //到达终点
                printf("到达终点：(%3d,%3d)\n",e.pos.x, e.pos.y);
                return TRUE;
            }
            curpos = NextPos(curpos,1);   //下一位置是当前位置的东邻
            curstep++;  //探索下一步
            printf("step:%d",curstep);
        }//if
        else{  //当前位置不能通过
            if(!StackEmpty_L(S)){
                Pop_L(&S,&e);
                while(e.direction == 4 && !StackEmpty_L(S)){
                    MakePrint(e.pos);
                    Pop_L(&S,&e);
                    curstep--;
                    printf("倒退到(%3d,%3d)\n",e.pos.x,e.pos.y);
                }//while
                if(e.direction < 4){
                    ++e.direction;  //换下一个方向探索
                    printf("换方向\n",e.pos.x,e.pos.y);
                    Push_L(&S,e);
                    curpos = NextPos(e.pos,e.direction); //设定当前位置是该新方向上的相邻块
                }//if
            }//if
        }//else
    }while(!StackEmpty_L(S));
    if(StackEmpty_L(S))
    {
        printf("对不起，找不到出口\n");
    }
    return FALSE;
}*/