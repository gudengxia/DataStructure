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