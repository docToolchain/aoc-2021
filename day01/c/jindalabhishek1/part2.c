#include <stdio.h>
#include <stdlib.h>

// usage: ./part2 input.txt
int main(int argc, char *argv[])
{
    char *filename;

    // checking if filename is passed as cmd arg
    if (argc == 2)
    {
        filename = argv[1];
    }
    else
    {
        filename = "./test-input.txt";
    }

    // opening file
    FILE *fp = fopen(filename, "r");

    // checking if the file is opened correctly
    if (fp == NULL)
    {
        printf("File not found!\n");
        return -1;
    }

    int size = 0;
    int temp = 0;

    /* 
        Getting the number of integers in the file
    */
    while ((fscanf(fp, "%d", &temp)) != EOF)
    {
        size++;
    }

    // printf("Size: %i\n", size);
    
    // checking if we have less than 3 lines in input
    if (size < 3)
    {
        printf("We need more data\n");
        return 1;
    }
    
    if (feof(fp))
    {
        // Settig the stream pointer to start of the file stream
        // SEEK_SET = 0
        fseek(fp, 0, SEEK_SET);
    }

    // dynamically allocating array
    int *arr = malloc (size * sizeof(int));
    int i = 0;

    // scaning data into the array
    while (!feof(fp))
    {
        int temp = 0;
        fscanf(fp, "%d", &arr[i]);
        i++;
    }

    // closing the file
    fclose(fp);

    int prev_sum = 0;
    i = 0;
    int count = 0;

    while (i < size - 2)
    {
        // calculating sum of consecitive 3 measuments
        int curr_sum = arr[i] + arr[i+1] + arr[i+2];
        
        /*
            if there are only 3 int
            we will continue to next cycle
        */
        if (prev_sum == 0)
        {
            prev_sum = curr_sum;
            curr_sum = 0;
        }
        else 
        {
            if (curr_sum > prev_sum)
            {
                count++;
            }
            prev_sum = curr_sum;
        }
        i++;
    }

    printf ("Count: %d\n", count);
    return 0;
}