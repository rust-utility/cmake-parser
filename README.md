# CMake Parser for Rust

`cmake-parser` is a Rust library that provides a set of tools for parsing CMake files and working with the data they contain. The library includes a parser for reading CMake files, as well as several structs and enums for representing the data defined in CMake files.

CMake version: [v3.26](https://cmake.org/cmake/help/v3.26/index.html)

CMake Language specification:

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

Implemented: 92 of 127.

### Scripting Commands

These commands are always available.

- [x] block
- [x] break
- [x] cmake_host_system_information
- [x] cmake_language
- [x] cmake_minimum_required
- [x] cmake_parse_arguments
- [x] cmake_path
- [x] cmake_policy
- [x] configure_file
- [x] continue
- [x] else
- [x] elseif
- [x] endblock
- [x] endforeach
- [x] endfunction
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

These commands are available only in CMake projects.

- [x] add_compile_definitions
- [x] add_compile_options
- [x] add_custom_command
- [x] add_custom_target
- [x] add_definitions
- [x] add_dependencies
- [x] add_executable
- [x] add_library
- [x] add_link_options
- [x] add_subdirectory
- [x] add_test
- [x] aux_source_directory
- [x] build_command
- [x] create_test_sourcelist
- [x] define_property
- [x] enable_language
- [x] enable_testing
- [x] export
- [x] fltk_wrap_ui
- [x] get_source_file_property
- [x] get_target_property
- [x] get_test_property
- [x] include_directories
- [x] include_external_msproject
- [x] include_regular_expression
- [x] install
- [x] link_directories
- [x] link_libraries
- [x] load_cache
- [x] project
- [x] remove_definitions
- [x] set_source_files_properties
- [x] set_target_properties
- [x] set_tests_properties
- [x] source_group
- [x] target_compile_definitions
- [x] target_compile_features
- [x] target_compile_options
- [x] target_include_directories
- [x] target_link_directories
- [x] target_link_libraries
- [x] target_link_options
- [x] target_precompile_headers
- [x] target_sources
- [x] try_compile
- [x] try_run

### CTest Commands

These commands are available only in CTest scripts.

- [x] ctest_build
- [x] ctest_configure
- [x] ctest_coverage
- [x] ctest_empty_binary_directory
- [x] ctest_memcheck
- [x] ctest_read_custom_files
- [x] ctest_run_script
- [x] ctest_sleep
- [x] ctest_start
- [x] ctest_submit
- [x] ctest_test
- [x] ctest_update
- [x] ctest_upload

### Deprecated Commands

These commands are deprecated and are only made available to maintain backward compatibility. The documentation of each command states the CMake version in which it was deprecated. Do not use these commands in new code.

- [x] build_name
- [x] exec_program
- [x] export_library_dependencies
- [x] install_files
- [x] install_programs
- [x] install_targets
- [x] load_command
- [x] make_directory
- [x] output_required_files
- [x] qt_wrap_cpp
- [x] qt_wrap_ui
- [x] remove
- [x] subdir_depends
- [x] subdirs
- [x] use_mangled_mesa
- [x] utility_source
- [x] variable_requires
- [x] write_file
