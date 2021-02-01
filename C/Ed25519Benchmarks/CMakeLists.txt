cmake_minimum_required( VERSION 3.16 )

project( Ed25519Benchmarks VERSION 0.1.0 )

add_subdirectory( benchmark )
add_subdirectory( src )
# find_package(CryptoPP REQUIRED)