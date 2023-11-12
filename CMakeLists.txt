cmake_minimum_required(VERSION 3.20)

project(StepMania)

include(StepmaniaCore.cmake)

# The external libraries need to be included.
add_subdirectory(extern)

# Our custom Rust hook
add_subdirectory(rust)

# The internal libraries and eventual executable to be used.
add_subdirectory(src)
