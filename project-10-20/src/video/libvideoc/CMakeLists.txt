#See https://stackoverflow.com/questions/17511496/how-to-create-a-shared-library-with-cmake

cmake_minimum_required(VERSION 3.9)

project(videoc VERSION 0.1.0 DESCRIPTION "Some stub library")

SET(INSTALLDIR "${CMAKE_CURRENT_SOURCE_DIR}/install" CACHE STRING "install directory for the headers and library")

add_library(videoc SHARED
src/videoc.c
)

set_target_properties(videoc PROPERTIES VERSION ${PROJECT_VERSION})
set_target_properties(videoc PROPERTIES PUBLIC_HEADER include/videoc.h)

configure_file(videoc.pc.in videoc.pc @ONLY)

target_include_directories(videoc PRIVATE include)
target_include_directories(videoc PRIVATE src)

install(TARGETS videoc
LIBRARY DESTINATION ${INSTALLDIR}/libs
PUBLIC_HEADER DESTINATION ${INSTALLDIR}/include)

install(FILES ${CMAKE_BINARY_DIR}/videoc.pc
DESTINATION ${INSTALLDIR}/pkgconfig)

