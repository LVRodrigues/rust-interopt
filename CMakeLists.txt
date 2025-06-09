cmake_minimum_required(VERSION 3.25.0)
project(demo VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(PACKAGE_BUGREPORT "Luciano Vieira Rodrigues <luciano@codesolver.com.br>")

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    #set(CARGO_CMD RUSTFLAGS=-Zsanitizer=address cargo build -Zbuild-std --target x86_64-unknown-linux-gnu)
    #set(TARGET_DIR "x86_64-unknown-linux-gnu/debug")
    set(CARGO_CMD RUSTFLAGS=-Zsanitizer=address cargo build --debug)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()
SET(LIB_FILE "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libdemo.so")

add_custom_command(OUTPUT ${LIB_FILE}
        COMMENT "Compiling rust module"
        COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/lib)
        
add_custom_target(rust_lib_target DEPENDS ${LIB_FILE})
add_library(rust_lib STATIC IMPORTED GLOBAL)
add_dependencies(rust_lib rust_lib_target)

# include_directories(../lib/target/include)
# add_executable(demo app/main.cpp)
# target_link_directories(demo PRIVATE ../lib/target/debug)
# target_link_libraries(demo PRIVATE libdemo.so)


