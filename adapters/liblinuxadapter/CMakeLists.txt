cmake_minimum_required(VERSION 3.0)
project(liblinuxadapter)

set(CMAKE_REQUIRED_INCLUDES "/usr/local/include" "/usr/include")

add_library(linuxadapter STATIC adapter.c adapter.h)

install(TARGETS linuxadapter DESTINATION .)
