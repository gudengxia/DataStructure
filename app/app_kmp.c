#include <stdio.h>
#include <stdlib.h>
#include "kmp.h"
int main()
{
  int i;
  char *S = "ababcabcacbab";
  char *T = "abcac";
  int n = 5;
  int *next = (int*)malloc(sizeof(int) * n);

  get_next(T, next);

  for(i = 0; i < n; i++)
    {
      printf("%3d", next[i]);
    }
  printf("\n");

  i = index_KMP(S, T, 0);
  free(next);
  printf("i = %d\n", i);
  return 0;
}
