#include<stdio.h>
#include <stdlib.h>
#include <time.h>

void fill_random(int array[], int length, int max);

void quicksort(int number[25],int first,int last){
    int i, j, pivot, temp;
    if(first<last){
        pivot=first;
        i=first;
        j=last;
        while(i<j){
            while(number[i]<=number[pivot]&&i<last)
            i++;
            while(number[j]>number[pivot])
            j--;
            if(i<j){
                temp=number[i];
                number[i]=number[j];
                number[j]=temp;
            }
        }
        temp=number[pivot];
        number[pivot]=number[j];
        number[j]=temp;
        quicksort(number,first,j-1);
        quicksort(number,j+1,last);
    }
}


void fill_random(int array[], int length, int max){
    for (int i = 0; i < length; i++)
        array[i] = (rand() % max)+1;
}


int main(){
    int size = 3000000;
    int a[size];
    fill_random(a,size,1000000);
    int i, count;
    count = size;
    clock_t t;
    t = clock();

    quicksort(a,0,count-1);
    t = clock() - t;
    double time_taken = ((double)t)/CLOCKS_PER_SEC; // calculate the elapsed time
    printf("The program took %f seconds to execute", time_taken);
    return 0;
}