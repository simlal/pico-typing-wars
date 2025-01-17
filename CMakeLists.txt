cmake_minimum_required(VERSION 3.13...3.27)

# initialize the SDK based on PICO_SDK_PATH
include(pico_sdk_import.cmake)

project(typing_wars)

# Board type, debug and IDE
set(PICO_BOARD pico)
set(CMAKE_BUILD_TYPE debug)
set(CMAKE_EXPORT_COMPILE_COMMANDS on)

pico_sdk_init()

add_executable(typing_wars src/main.c)

# pull in common dependencies
target_link_libraries(typing_wars pico_stdlib)

# create map/bin/hex/uf2 file etc.
pico_add_extra_outputs(typing_wars)
