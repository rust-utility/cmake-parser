# ros-cmake-parser

`ros-cmake-parser` is based on the original `cmake-parser` project and includes additional changes to better support ROS-oriented CMake usage, especially around ament/catkin-related command handling and raw command access for downstream tooling.

This repository contains a Rust workspace with:

- `ros-cmake-parser` — the parser library
- `ros-cmake-parser-derive` — derive macros used by the parser crate

CMake version: [v3.26](https://cmake.org/cmake/help/v3.26/index.html)

CMake Language specification:

<https://cmake.org/cmake/help/v3.26/manual/cmake-language.7.html>
