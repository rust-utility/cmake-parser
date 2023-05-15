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

## Supported Commands

### Scripting Commands

- [ ] block
- [ ] break
- [ ] cmake_host_system_information
- [ ] cmake_language
- [ ] cmake_minimum_required
- [ ] cmake_parse_arguments
- [ ] cmake_path
- [ ] cmake_policy
- [ ] configure_file
- [ ] continue
- [ ] else
- [ ] elseif
- [ ] endblock
- [ ] endforeach
- [ ] endfunction
- [ ] endif
- [ ] endmacro
- [ ] endwhile
- [ ] execute_process
- [ ] file
- [ ] find_file
- [ ] find_library
- [ ] find_package
- [ ] find_path
- [ ] find_program
- [ ] foreach
- [ ] function
- [ ] get_cmake_property
- [ ] get_directory_property
- [ ] get_filename_component
- [ ] get_property
- [ ] if
- [ ] include
- [ ] include_guard
- [ ] list
- [ ] macro
- [ ] mark_as_advanced
- [ ] math
- [ ] message
- [ ] option
- [ ] return
- [ ] separate_arguments
- [ ] set
- [ ] set_directory_properties
- [ ] set_property
- [ ] site_name
- [ ] string
- [ ] unset
- [ ] variable_watch
- [ ] while

### Project Commands

- [x] add_compile_definitions
- [x] add_compile_options
- [x] add_custom_command
- [x] add_custom_target
- [ ] add_definitions
- [ ] add_dependencies
- [ ] add_executable
- [ ] add_library
- [ ] add_link_options
- [ ] add_subdirectory
- [ ] add_test
- [ ] aux_source_directory
- [ ] build_command
- [ ] create_test_sourcelist
- [ ] define_property
- [ ] enable_language
- [ ] enable_testing
- [ ] export
- [ ] fltk_wrap_ui
- [ ] get_source_file_property
- [ ] get_target_property
- [ ] get_test_property
- [ ] include_directories
- [ ] include_external_msproject
- [ ] include_regular_expression
- [ ] install
- [ ] link_directories
- [ ] link_libraries
- [ ] load_cache
- [ ] project
- [ ] remove_definitions
- [ ] set_source_files_properties
- [ ] set_target_properties
- [ ] set_tests_properties
- [ ] source_group
- [ ] target_compile_definitions
- [ ] target_compile_features
- [ ] target_compile_options
- [ ] target_include_directories
- [ ] target_link_directories
- [ ] target_link_libraries
- [ ] target_link_options
- [ ] target_precompile_headers
- [ ] target_sources
- [ ] try_compile
- [ ] try_run
