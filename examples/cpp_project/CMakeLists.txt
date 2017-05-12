cmake_minimum_required (VERSION 2.6)
project(cpp_example)

include_directories(include)
add_library(cpp_example SHARED src/src1.cpp)
