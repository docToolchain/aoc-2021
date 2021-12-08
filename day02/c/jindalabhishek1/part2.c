// Question: https://adventofcode.com/2021/day/2#part2

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char const *argv[])
{
    char const *filename;
    if (argc < 2)
    {
        printf("Continuing with default file name: test-input.txt\n");
        filename = "test-input.txt";
    }
    else
    {
        filename = argv[1];
    }

    FILE *file = fopen(filename, "r");

    if (file == NULL)
    {
        printf("File not found\n");
        return 2;
    }

    int horizontal_position = 0;
    int depth = 0;
    int aim = 0;

    while (feof(file) == 0)
    {
        char *direction = malloc(8 * sizeof(char));
        int movement = 0;
        fscanf(file, "%s %d", direction, &movement);
        
        if (strcmp(direction, "down") == 0)
        {
            aim += movement;
        }
        else if (strcmp(direction, "up") == 0)
        {
            aim -= movement;
        }
        else if (strcmp(direction, "forward") == 0)
        {
            horizontal_position += movement;
            depth += aim * movement;
        }
        
        free(direction);
    }

    fclose(file);

    printf("Final position: \nHorizontal: %d\tDepth: %d\n", horizontal_position, depth);

    printf("Product: %d\n", horizontal_position * depth);

    return 0;
}