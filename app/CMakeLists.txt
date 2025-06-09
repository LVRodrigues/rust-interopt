cmake_minimum_required(VERSION 3.25.0)
project(demo VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(PACKAGE_BUGREPORT "Luciano Vieira Rodrigues <luciano@codesolver.com.br>")

# if (CMAKE_BUILD_TYPE STREQUAL "Debug")
#     #set(CARGO_CMD RUSTFLAGS=-Zsanitizer=address cargo build -Zbuild-std --target x86_64-unknown-linux-gnu)
#     #set(TARGET_DIR "x86_64-unknown-linux-gnu/debug")
#     set(CARGO_CMD RUSTFLAGS=-Zsanitizer=address cargo build --manifest-path ../lib/Cargo.toml --debug)
#     set(TARGET_DIR "debug")
# else ()
#     set(CARGO_CMD cargo build --manifest-path ../lib/Cargo.toml --release)
#     set(TARGET_DIR "release")
# endif ()
# SET(LIB_FILE "../lib/${TARGET_DIR}/libdemo.so")

include_directories(../lib/target/include)
add_executable(demo src/main.cpp)
target_link_directories(demo PRIVATE ../lib/target/debug)
target_link_libraries(demo PRIVATE libdemo.so)


