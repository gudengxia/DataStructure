#include <stdio.h>

int sum(int x, int y){
    int c;
    c = x + y;
    return c;
}

int main(){
    int a = 3; 
    int b = 4;
    int c = sum(a, b);
    printf("sum=%d\n", c);
    return 0;
}