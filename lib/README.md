# ros-cmake-parser

`ros-cmake-parser` is based on the original `cmake-parser` project and includes additional changes to better support ROS-oriented CMake usage, especially around ament/catkin-related command handling and raw command access for downstream tooling.

CMake version: [v3.26](https://cmake.org/cmake/help/v3.26/index.html)

CMake Language specification:

<https://cmake.org/cmake/help/v3.26/manual/cmake-language.7.html>

## Features

The `ros-cmake-parser` library provides the following features:

- Parsing of `CMakeLists.txt` files
- Typed parsing for standard CMake commands
- Raw command access for commands not modeled as typed commands
- ROS-oriented helpers for key commands such as `ament_package`, `catkin_package`, and `ament_target_dependencies`
- Error handling for parsing failures

## Usage

Add dependency to `Cargo.toml`:

```toml
[dependencies]
ros-cmake-parser = "0.1"
```

Example `src/main.rs`:

```rust
use ros_cmake_parser::{parse_cmakelists, Command, Doc};

let cmakelists = br#"
add_custom_command(
  TARGET myExe POST_BUILD
  COMMAND someHasher -i "$<TARGET_FILE:myExe>"
                      -o "$<TARGET_FILE:myExe>.hash"
  VERBATIM)
"#;

let cmakelists = parse_cmakelists(cmakelists).expect("valid CMakeLists.txt");
let doc = Doc::from(cmakelists);
let commands = doc.commands().expect("valid CMake commands");
assert!(matches!(
    commands.as_slice(),
    [Command::AddCustomCommand(_)]
));
dbg!(commands);
```
