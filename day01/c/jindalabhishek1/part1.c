// Question: https://adventofcode.com/2021/day/1

#include <stdio.h>


int main(void)
{
    FILE *fp = fopen("./input.txt", "r");
    if (fp == NULL)
    {
        printf("File not found\n");
        return -1;
    }

    int prev;

    fscanf(fp, "%d", &prev);
    
    // printf ("prev: %d\n", prev);
    int count = 0;
    int curr = 0;
    while (feof(fp) == 0)
    {
        fscanf(fp, "%d", &curr);
        if (curr > prev)
        {
            count++;
        }
        prev = curr;
    }

    fclose(fp);
    
    printf("Count: %d\n", count);


    
}