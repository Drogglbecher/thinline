cmake_minimum_required (VERSION 2.6)
project(c_example)

include_directories(include)
add_library(c_example SHARED src/src1.c src/src2.c)
