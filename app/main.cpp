#include <iostream>
#include <cstring>
#include <cstdlib>
#include "demo.h"

using namespace codesolver;

int main() {
    VersionText text;
    version(&text);
    std::cout << text.text <<  std::endl;

    return EXIT_SUCCESS;
}