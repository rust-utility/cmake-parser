# ros-cmake-parser-derive

`ros-cmake-parser-derive` is the derive-macro companion crate for `ros-cmake-parser`. It is based on the original `cmake-parser` derive crate and is distributed together with ROS-oriented parser extensions.

This crate provides derive macros for generating parsing code for CMake token structures and enums.

## Usage

```rust
use ros_cmake_parser_derive::CMake;
use ros_cmake_parser::Token;

#[derive(CMake)]
struct MyCMakeData<'t> {
    output: Token<'t>,
    depends: Option<Vec<Token<'t>>>,
}
```

Add dependencies to `Cargo.toml`:

```toml
[dependencies]
ros-cmake-parser = "0.1"
ros-cmake-parser-derive = "0.1"
```
