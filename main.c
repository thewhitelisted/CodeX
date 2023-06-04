#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <conio.h>
#include <process.h>

int create_C(char name[]);

int main(void)
{
    system("cls");
    char input[255];
    char folder_path[255];
    char file_path[255];

    while (1)
    {
        printf("Code File Creator\n\n");
        printf("Commands: c, py\n\n");
        scanf("%s", input);

        if (!strcmp(input, "c"))
        {
            system("cls");
            printf("C FILE CREATOR");

            printf("\n\nENTER THE FOLDER PATH: ");
            scanf("%s", folder_path);

            if (folder_path[strlen(folder_path) - 1] == '\\')
            {
                folder_path[strlen(folder_path) - 1] = '\0';
            }

            printf("\nENTER THE FILE NAME (DO NOT INCLUDE FILE EXTENSION): ");
            scanf("%s", file_path);

            strcat(strcat(folder_path, "\\"), file_path);

            if (create_C(folder_path) == 1)
            {
                printf("\nSuccessfully created file at %s.c", folder_path);
            }
            else
            {
                printf("\nCould not create file");
            }
            printf("\npress any key to continue");
            getch();
            system("cls");
            strcat(folder_path, ".c");
            int cmd = _execlp("code",
                   "code", 
                   folder_path,
                   NULL);
        }
        else
        {
            system("cls");
        }
    }

    return 0;
}

int create_C(char name[])
{
    char file_name[255];
    const char file_extension[] = ".c";
    strcpy(file_name, name);
    strcat(file_name, file_extension);
    FILE *fp;
    fp = fopen(file_name, "w");

    fputs("#include <stdio.h>\n\nint main()\n{\n\t\n\treturn 0;\n}", fp);

    FILE *file_check;
    if (file_check = fopen(file_name, "r"))
    {
        fclose(fp);
        return 1;
    }
    return 0;
}
