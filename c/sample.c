#include "./cffi.h"
#include <stdio.h>

int main(void) {
    int a = temp();
    printf("Function result: %d\n", a);
    MyStruct c = { 1 };
    MyResult b = some_method(&c);
    printf("Struct method result: %d\n", b.a);
}