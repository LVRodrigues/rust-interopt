#include <iostream>
#include <cstring>
#include "rust-interop.h"

using namespace codesolver;

int main() {
    char buffer[512];
    short size = 512;
    version(buffer, size);
    
    std::cout << "C: " <<   std::string(buffer) << std::endl;
    return 0;
}