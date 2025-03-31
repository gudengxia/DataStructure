#include "kmp.h"
#include <stdlib.h>
#include <string.h>
void get_next(char T[], int next[])
{
  int i = 0;
  int j = -1;
  next[0] = -1;
  while(T[i] != '\0')
    {
      if(j == -1 || T[i] == T[j])
        {
          ++i;
          ++j;
          if(T[i] != T[j])
            next[i] = j;
          else
            next[i] = next[j];
        }
        else
        {
          j = next[j];
        }
    }
}

int index_KMP(char* S, char* T, int pos)
{
  int i = pos;
  int j = 0;

  int *next = (int *)malloc(sizeof(int) * strlen(T));
  get_next(T, next);
  while(S[i] != '\0' && T[j] != '\0')
    {
      if(j == -1 || S[i] == T[j])
        {
	  i++;
	  j++;
        }
      else
        {
	  j = next[j];
        }
    }
  free(next);
  if(T[j] == '\0')
    return i - j;
  else
    return -1;
}
