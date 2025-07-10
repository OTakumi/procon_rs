cmake_minimum_required(VERSION {{CMAKE_VERSION}})
project({{PROJECT_NAME}})

set(CMAKE_CXX_STANDARD {{CPP_STANDARD}})
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable({{PROJECT_NAME}} main.cpp)

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    target_compile_options({{PROJECT_NAME}} PRIVATE -g -O0 -Wall -Wextra)
else()
    target_compile_options({{PROJECT_NAME}} PRIVATE -O2 -DNDEBUG)
endif()