# CMake Parser for Rust

`cmake-parser` is a Rust library that provides a set of tools for parsing CMake files and working with the data they contain. The library includes a parser for reading CMake files, as well as several structs and enums for representing the data defined in CMake files.

CMake version: [v3.26](https://cmake.org/cmake/help/v3.26/index.html)

cmake-language specification:

<https://cmake.org/cmake/help/v3.26/manual/cmake-language.7.html>

## Features

The `cmake-parser` library provides the following features:

- Parsing of `CMakeLists.txt` files: The library includes a parser for reading `CMakeLists.txt` files and extracting the data defined in them.
- Error handling: The library provides a set of error types for handling errors that may occur during parsing and processing of CMake files.

## Usage

Add dependency to `Cargo.toml`:

```toml
[dependencies]
cmake-parser = "0.1"
```

Example `src/main.rs`:

```rust
use cmake_parser::{parse_cmakelists, Doc};

let cmakelists = br#"""
add_custom_command(
  TARGET myExe POST_BUILD
  COMMAND someHasher -i "$<TARGET_FILE:myExe>"
                      -o "$<TARGET_FILE:myExe>.hash"
  VERBATIM)
"""#;

let cmakelists = parse_cmakelists(cmakelists).expect("valid CMakeLists.txt");
let doc = Doc::from(cmakelists);
let commands = doc.commands().expect("valid CMake commands");
dbg!(commands);
```
