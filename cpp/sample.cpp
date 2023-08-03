#include <iostream>
#include "./cppffi.h"

int main() {
    std::cout << "Result of rust call: " << temp() << std::endl;
}