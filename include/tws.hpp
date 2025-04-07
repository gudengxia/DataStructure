
#ifndef TWS_HPP
#define TWS_HPP
#include <iostream>
using namespace std;

#define OK 1
#define ERROR 0
#define OVERFLOW -2
typedef int status;

const int MAXSIZE = 20;
template <class T> class tws
{
public:
	tws();
	tws(int n);
	~tws();
	bool isEmpty(int i);
	bool isEmpty();
	bool isFull();
	status GetTop(int i, T& e);
	status push(int i, T e);
	status pop(int i, T& e);
private:
	T* base[2];
	T* top[2];
};

template <class T> tws<T>::tws()
{
	base[0] = top[0] = new T[MAXSIZE];
	if(!base[0])
	{
		cerr<<"Error: memory allocate error."<<endl;
		exit(OVERFLOW);
	}
	base[1] = top[1] = &base[0][MAXSIZE-1];
}

template <class T> tws<T>::tws(int n)
{
	base[0] = top[0] = new T[n];
	if(!base[0])
	{
		cerr<<"Error: memory allocate error."<<endl;
		exit(OVERFLOW);
	}
	base[1] = top[1] = &base[0][n-1];
}

template <class T> tws<T>::~tws()
{
	delete[] base[0];
}

template <class T> bool tws<T>::isEmpty(int i)
{
	if((i==1&&top[0]==base[0]) || (i==2&&top[1]==base[1]))
		return true;
	else
		return false;

}

template <class T> bool tws<T>::isEmpty()
{
	if(top[0] == base[0] && top[1] == base[1])
		return true;
	else
		return false;
}

template <class T> bool tws<T>::isFull()
{
	if(top[0] == top[1])
		return true;
	else
		return false;
}

template <class T> status tws<T>::GetTop(int i, T& e)
{
	if(i==1 && !isEmpty(1))
	{
		e = top[0];
		return OK;
	}
	if(i==2 && !isEmpty(2))
	{
		e = top[1];
		return OK;
	}

	return ERROR;
}

template <class T> status tws<T>::push(int i, T e)
{
	if(this->isFull() || (i != 1 && i != 2))
	{
		cerr<<"Error: stack is full or undefined index"<<endl;
		return ERROR;
	}
	
	if(i == 1)
	{
		*top[0] = e;
		top[0]++;
	}
	else
	{
		*top[1] = e;
		top[1]--;
	}
	return OK;
}

template <class T> status tws<T>::pop(int i, T& e)
{
	if((i==1&&base[0]==top[0]) || (i==2&&base[1]==top[1]))
	{
		cerr<<"Error: stack is empty or undefined index."<<endl;
		return ERROR;
	}
	
	if(i == 1)
		e = *(--top[0]);
	else
		e = *(++top[1]);
	return OK;
}
#endif
