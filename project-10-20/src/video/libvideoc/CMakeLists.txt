#See https://stackoverflow.com/questions/17511496/how-to-create-a-shared-library-with-cmake

cmake_minimum_required(VERSION 3.9)

find_package(PkgConfig REQUIRED)

project(videoc VERSION 0.1.0 DESCRIPTION "High level abstraction over FFMPEG")

if(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
  set(CMAKE_INSTALL_PREFIX "${CMAKE_BINARY_DIR}/install" CACHE PATH "install directory for the headers and library" FORCE)
endif()

include(GNUInstallDirs)


# Library #############################
SET(SRC_FILES "src/videoc.c;src/renderframe.c")
SET(HEADER_FILES "include/videoc.h;include/renderframe.h")

# add_library(videoc SHARED ${SRC_FILES})
add_library(videoc STATIC ${SRC_FILES})

set_target_properties(videoc PROPERTIES VERSION ${PROJECT_VERSION})
set_target_properties(videoc PROPERTIES PUBLIC_HEADER "${HEADER_FILES}")

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
LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})

install(FILES ${CMAKE_BINARY_DIR}/videoc.pc DESTINATION ${CMAKE_INSTALL_DATAROOTDIR}/pkgconfig)

# Debug ###############################

add_executable(debug EXCLUDE_FROM_ALL "src/debug_main.c")
target_include_directories(debug PRIVATE include)
target_link_libraries(debug videoc)

