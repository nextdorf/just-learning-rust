#See https://stackoverflow.com/questions/17511496/how-to-create-a-shared-library-with-cmake

cmake_minimum_required(VERSION 3.9)

find_package(PkgConfig REQUIRED)

project(videoc VERSION 0.1.0 DESCRIPTION "Some stub library")

SET(INSTALLDIR "${CMAKE_BINARY_DIR}/install" CACHE STRING "install directory for the headers and library")

SET(SRC_FILES "src/videoc.c;src/renderframe.c")
SET(HEADER_FILES "include/videoc.h;include/renderframe.h")

add_library(videoc SHARED ${SRC_FILES})

set_target_properties(videoc PROPERTIES VERSION ${PROJECT_VERSION})
set_target_properties(videoc PROPERTIES PUBLIC_HEADER "${HEADER_FILES}")
# set_target_properties(videoc PROPERTIES PUBLIC_HEADER include/renderframe.h)

configure_file(videoc.pc.in videoc.pc @ONLY)


pkg_check_modules(LIBAVFORMAT REQUIRED IMPORTED_TARGET libavformat)
pkg_check_modules(LIBAVUTIL REQUIRED IMPORTED_TARGET libavutil)
pkg_check_modules(LIBSWSCALE REQUIRED IMPORTED_TARGET libswscale)
pkg_check_modules(LIBAVCODEC REQUIRED IMPORTED_TARGET libavcodec)

target_link_libraries(videoc #-static
PkgConfig::LIBAVUTIL
PkgConfig::LIBAVCODEC
PkgConfig::LIBAVFORMAT
PkgConfig::LIBSWSCALE
)


target_include_directories(videoc PRIVATE include)
target_include_directories(videoc PRIVATE src)

install(TARGETS videoc
LIBRARY DESTINATION ${INSTALLDIR}/libs
PUBLIC_HEADER DESTINATION ${INSTALLDIR}/include)

install(FILES ${CMAKE_BINARY_DIR}/videoc.pc
DESTINATION ${INSTALLDIR}/pkgconfig)

