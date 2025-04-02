#include<stdio.h>
extern int multiply(int a,int b);
int main(){
    int result = multiply(8,9);
    printf("Result is %d\n",result);
    return 0;
}