cmake_minimum_required(VERSION 3.16)

project(cxx LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(fmt REQUIRED)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

file(GLOB_RECURSE SRC_FILES CONFIGURE_DEPENDS
    "${CMAKE_SOURCE_DIR}/src/*.cc"
)

foreach(SRC ${SRC_FILES})
    get_filename_component(TARGET_NAME ${SRC} NAME_WE)

    add_executable(${TARGET_NAME} ${SRC})
    target_link_libraries(${TARGET_NAME} PRIVATE fmt::fmt)
endforeach()