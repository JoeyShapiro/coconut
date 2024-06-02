#include <stdio.h>

int main(int argc, char const *argv[])
{
    // tupe punning
    float d = 42.0;
    printf("%d\n", *(int*)&d);
    return 0;
}
