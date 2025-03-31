#include "EQueen.h"
#include <cstring>
#include <stack>
#include <iostream>
using namespace std;

int EQueen::solve()
{
    int num_of_solution = 0;
    stack<Loc> s;

    Loc cur, nextpos;
    cur.i = cur.j = 0;
    board[cur.i][cur.j] = true;
    s.push(cur);
    cout<<"Push:("<<cur.i<<","<<cur.j<<")"<<endl;
    while(!s.empty())
    {
        cur = s.top();
        //1. Have placed the last row
        if(cur.i == n - 1)
        {
            PrintBoard();
            //break;
            //find next solution
            num_of_solution += 1;
            board[cur.i][cur.j] = false;
            s.pop();

            while(!next_safe_pos_in_same_row(cur, &nextpos) && !s.empty())
            {
                cur = s.top();
                s.pop();
                board[cur.i][cur.j] = false;
                cout<<"Pop:("<<cur.i<<","<<cur.j<<")"<<endl;
            }

            if(next_safe_pos_in_same_row(cur, &nextpos))// if so, put this element into the stack
            {
                s.push(nextpos);
                board[nextpos.i][nextpos.j] = true;
                cout<<"Push:("<<nextpos.i<<","<<nextpos.j<<")"<<endl;
            }
        }

        //2. find a valid position in the next row
        if(next_safe_pos_in_next_row(cur, &nextpos))
        {
            cur = nextpos;
            board[cur.i][cur.j] = true;
            s.push(cur);
            cout<<"Push:("<<cur.i<<","<<cur.j<<")"<<endl;
        }
        else //backtrack
        {
            board[cur.i][cur.j] = false;
            s.pop();
            cout<<"Pop:("<<cur.i<<","<<cur.j<<")"<<endl;
            //2-1 Backtrack until there is another possibility for the element just popped
            while(!next_safe_pos_in_same_row(cur, &nextpos) && !s.empty())
            {
                cur = s.top();
                s.pop();
                board[cur.i][cur.j] = false;
                cout<<"Pop:("<<cur.i<<","<<cur.j<<")"<<endl;
            }

            if(next_safe_pos_in_same_row(cur, &nextpos))// if so, put this element into the stack
            {
                s.push(nextpos);
                board[nextpos.i][nextpos.j] = true;
                cout<<"Push:("<<nextpos.i<<","<<nextpos.j<<")"<<endl;
            }
        }

    }
    return num_of_solution;
}

void EQueen::PrintBoard()
{
    int i, j;
    for(i = 0; i < n; i++)
    {
        for(j = 0; j < n; j++)
        {
            if(board[i][j])
                cout<<"Q"<<ends;
            else
                cout<<"-"<<ends;
        }
        cout<<endl;
    }
}

EQueen::EQueen(int v)
{
    n = v;
    board = new bool*[n];
    int i;
    for(i = 0; i < n; i++)
    {
        board[i] = new bool[n];
        memset(board[i], false, sizeof(bool)*n);
    }
    //ctor
}

EQueen::~EQueen()
{
    //dtor
    int i;
    for(i = 0; i < n; i++)
    {
        delete[] board[i];
    }
    delete[] board;
}

bool EQueen::is_safe(Loc pos)
{
    int row = pos.i;
    int col = pos.j;

    int i, j;
    for(i = row-1; i >=0; i--)
    {
        if(board[i][col])
            return false;
    }
    for(j = col-1; j >= 0; j--)
    {
        if(board[row][j])
            return false;
    }

    i = row, j = col;
    while(i > 0 && j > 0)
    {
        i--, j--;
        if(board[i][j])
            return false;
    }

    i = row, j = col;
    while(i > 0 && j < n-1)
    {
        i--, j++;
        if(board[i][j])
            return false;
    }

    return true;
}

bool EQueen::next_safe_pos_in_same_row(Loc cur, Loc* pos)
{
    Loc p = cur;
    p.j++;
    while(p.j < n)
    {
        if(is_safe(p))
        {
            *pos = p;
            return true;
        }
        p.j++;
    }
    return false;
}

bool EQueen::next_safe_pos_in_next_row(Loc cur, Loc*pos)
{
    Loc p = cur;
    p.i++;
    p.j = 0;
     while(p.j < n)
    {
        if(is_safe(p))
        {
            *pos = p;
            return true;
        }
        p.j++;
    }
    return false;
}
