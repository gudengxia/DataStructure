#include "sort.h"

void InsertSort(ElemType a[], int n)
{
	int i, j;
	ElemType e;
	
	for(i = 1; i < n; i++)
	{
		if(a[i] < a[i-1])
		{
			e = a[i];
			j = i - 1;
			while(j >= 0 && a[j] > e)
			{
				a[j + 1] = a[j];
				j--;
			}	
			a[j+1] = e;
		}
	}
}

void BFInsertSort(ElemType a[], int n)
{
	int i, j;
	int low, high, mid;
	ElemType e;
	
	for(i = 1; i < n; i++)
	{
		e = a[i];
		low = 0, high = i-1;
		while(low <= high)
		{
			mid = (low + high) / 2;
			if(e < a[mid])
			{
				high = mid - 1;
			}
			else
			{
				low = mid + 1;
			}
		}
		
		for(j = i-1; j >= low; j--)
			a[j+1] = a[j];
		a[high+1] = e;
	}
}

void BinaryInsertSort(ElemType a[], int n)
{
	int i, j;
	int low, high, mid;
	ElemType e;
	
	for(i = 1; i < n; i++)
	{
		if(a[i-1] > a[i])
		{
			low = 0, high = i - 1;
			while(low < high)
			{
				mid = (low + high) / 2;
				if(a[mid] > a[i])
				{
					high = mid - 1;
				}
				else
				{
					low = mid + 1;
				}
			}
			
			e = a[i];
			for(j = i-1; j >= low; j--)
			{
				a[j+1] = a[j];
			}
			a[low] = e; 
		}
	}
}

void BubbleSort(ElemType a[], int n)
{
	int i, j;
	int cnt;
	ElemType e;
	
	for(i = 1; i < n; i++) 
	{
		cnt = 0;
		for(j = 1; j <= n-i; j++)
		{
			if(a[j-1] > a[j])
			{
				e = a[j-1];
				a[j-1] = a[j];
				a[j] = e;
				cnt++;
			}
		}
		
		if(cnt == 0)
		{
			break;
		}
	}
}

int Partition(ElemType a[], int low, int high)
{
	ElemType pivotkey = a[low];
	
	while(low < high)
	{
		while(low < high && a[high] >= pivotkey)
			high--;
		a[low] = a[high];
		while(low < high && a[low] <= pivotkey)
			low++;
		a[high] = a[low];
	}
	
	a[low] = pivotkey;
	return low;
}

void QSort(ElemType a[], int low, int high)
{
	int pivotloc;
	if(low < high)
	{
		pivotloc = Partition(a, low, high);
		QSort(a, low, pivotloc-1);
		QSort(a, pivotloc+1, high);
	}
}

void QuickSort(ElemType a[], int n)
{
	QSort(a, 0, n-1);
}

void SelectSort(ElemType a[], int n)
{
	int i, j;
	ElemType e, min_elem;
	int pmin;
	
	for(i = 0; i < n-1; i++)
	{
		min_elem = a[i];
		pmin = i;
		
		for(j = i+1; j < n; j++)
		{
			if(a[j] < min_elem)
			{
				min_elem = a[j];
				pmin = j;
			}
		}
		
		if(i != pmin)
		{
			e = a[i];
			a[i] = a[pmin];
			a[pmin] = e;
		}
	}
}

void HeapAdjust(ElemType a[], int s, int m)
{
	int j;
	ElemType e = a[s];
	for(j = (s+1)*2-1; j <= m; j = 2 * (j+1) -1)
	{
		if(j+1 <= m && a[j] < a[j+1]) 
			j++;
		if(e >= a[j]) break;
		a[s] = a[j]; 
		s = j;
	}
	a[s] = e;
}

void CreatHeap(ElemType a[], int n)
{
	int i;
	for(i = n/2-1; i >= 0; i--)
	{
		HeapAdjust(a, i, n-1);
	}
}

void HeapSort(ElemType a[], int n)
{
	int i;
	ElemType winner;
	
	CreatHeap(a, n-1);
	for(i = n-1; i > 0; i--)
	{
		winner = a[0];
		a[0] = a[i];
		a[i] = winner;
		HeapAdjust(a, 0, i-1);
	}
}