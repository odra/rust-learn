cmake_minimum_required(VERSION 3.5)
add_library (
    libmath 
    STATIC
    math.c
)

install (TARGETS libmath DESTINATION .)