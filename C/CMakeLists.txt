# Distributed under the MIT license
cmake_minimum_required(VERSION 3.8)

set(This Ed25519Benchmarks)

project(${This} C CXX VERSION 0.1)

set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

add_subdirectory(benchmark)
add_executable(${This} main.cc)
target_link_libraries(${This} PRIVATE benchmark::benchmark)