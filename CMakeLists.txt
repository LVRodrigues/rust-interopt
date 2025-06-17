cmake_minimum_required(VERSION 3.10.0)
project(csdemo VERSION 1.0.0 LANGUAGES C CXX)


if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()
set(LIB_FILE "${CMAKE_BINARY_DIR}/${TARGET_DIR}/lib${PROJECT_NAME}.so")
SET(LIB_HEADER_FOLDER "${CMAKE_SOURCE_DIR}/include")

add_custom_command(OUTPUT ${LIB_FILE}
        COMMENT "Compiling rust module"
        COMMAND CARGO_TARGET_DIR=${CMAKE_BINARY_DIR} ${CARGO_CMD}
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR})

add_custom_target(libcsdemo_target DEPENDS ${LIB_FILE})
add_library(libcsdemo SHARED IMPORTED GLOBAL)
add_dependencies(libcsdemo libcsdemo_target)
set_target_properties(libcsdemo
        PROPERTIES
        IMPORTED_LOCATION ${LIB_FILE}
        INTERFACE_INCLUDE_DIRECTORIES ${LIB_HEADER_FOLDER})

add_executable(csdemo src/main.cpp)
target_link_libraries(csdemo PUBLIC libcsdemo ${CMAKE_DL_LIBS})

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
