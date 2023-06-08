cmake_minimum_required(VERSION 3.10)
project("RayTrace C++")

file(GLOB source_files CONFIGURE_DEPENDS "${PROJECT_BINARY_DIR}/../src/*.cpp")
set(SOURCE main.cpp ${source_files})

add_executable(main ${SOURCE})
target_include_directories(main PUBLIC "${PROJECT_BINARY_DIR}/../headers")
include_directories("${PROJECT_BINARY_DIR}..//headers")