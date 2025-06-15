#include <iostream>
#include <cstddef>

#include "csdemo.h"

int main(int, char**){
    VersionInfo version = csdemo_version();

    std::cout << "Library.....: " << version.name << std::endl << 
                 "Version.....: " << (int) version.major << "." << (int) version.minor << "." << (int) version.patch << std::endl <<
                 "Description.: " << version.description << std::endl <<
                 "Author......: " << version.author << std::endl <<
                 "System......: " << version.system << std::endl <<
                 "Architecture: " << version.architecture << std::endl;

    return EXIT_SUCCESS;
}
