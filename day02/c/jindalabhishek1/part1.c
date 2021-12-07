// Problem: https://adventofcode.com/2021/day/2

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main (int argc, char *argv[])
{
    char *filename;
    if (argc < 2)
    {
        printf("Continuing with default input file: test-input.txt\n");
        filename = "test-input.txt";
    }
    else
    {
        filename = argv[1];
    }

    FILE *file = fopen(filename, "r");

    if (file == NULL)
    {
        printf("Error opening file!\n");
        return 1;
    }

    int horizontal_position = 0;
    int depth = 0;

    while (feof(file) == 0)
    {
        char *direction = malloc(8 * sizeof(char) * 2);
        int movement = 0;

        fscanf(file, "%s %d", &direction, &movement);
        // printf("Size of direction: %lu:%lu\n", strlen(direction), sizeof(direction));
        // fgetc(file);
        // printf("%c\t%d\n", c, c);
        printf("Direction: %s\tMovement: %d\n", direction, movement);
        if (strcmp(direction, "up") == 0)
        {
            depth -= movement;
        }
        else if (strcmp(direction, "down") == 0)
        {
            depth += movement;
        }
        else if (strcmp(direction, "forward") == 0)
        {
            horizontal_position += movement;
        }
        free(direction);
    }

    fclose(file);

    printf("Final position:\nHorizontal: %d\tVertical: %d\n", horizontal_position, depth);

    printf("Product: %d\n", horizontal_position * depth);
}