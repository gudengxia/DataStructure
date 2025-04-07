#ifndef TWS2QUEUE_HPP
#define TWS2QUEUE_HPP
#include "tws.hpp"

template <class T> class Queue:private tws<T>
{
public:
	Queue();
	Queue(int n);
	~Queue();
	bool isEmpty();
	bool isFull();
	status GetHead(T& e);
	status EnQueue(T e);
	status DeQueue(T& e);
};

template <class T> Queue<T>::Queue():tws<T>()
{
}

template <class T> Queue<T>::Queue(int n):tws<T>(n)
{
}

template <class T> Queue<T>::~Queue()
{
}

template <class T> bool Queue<T>::isEmpty()
{
	return tws<T>::isEmpty();
}

template <class T> bool Queue<T>::isFull()
{
	return tws<T>::isFull();
}

template <class T> status Queue<T>::GetHead(T &e)
{
	if(isEmpty())
	{
		cerr<<"Error: stack is empty!"<<endl;
		return ERROR;
	}

	if(isEmpty(2))
	{
		while(!isEmpty(1))
		{
			pop(1, e);
			push(2, e);
		}
	}
		
	GetTop(2, e);
	return OK;
}

template <class T> status Queue<T>::EnQueue(T e)
{
	if(isFull())
	{
		cerr<<"Error: queue is full!"<<endl;
		return ERROR;
	}

	push(1, e);
	return OK;
}

template <class T> status Queue<T>::DeQueue(T& e)
{
	if(isEmpty())
	{
		cerr<<"Error: queue is empty!"<<endl;
		return ERROR;
	}

	if(tws<T>::isEmpty(2))
	{
		while(!tws<T>::isEmpty(1))
		{
			pop(1, e);
			push(2, e);
		}
	}
		
	pop(2, e);
	return OK;
}
#endif

