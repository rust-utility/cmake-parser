# cmake-parser-derive

A set of Rust derive macros for parsing CMake tokens to Rust structures and enums.

This library provides a set of derive macros that can be used to automatically generate Rust code for parsing CMake tokens. These macros can be applied to Rust structures and enums to generate the necessary parsing code at compile time, greatly simplifying the process of working with CMake data in Rust.

To use this library, simply add it to your project's dependencies in `Cargo.toml`, and then import the necessary derive macros into your Rust code. For example:

Once you've imported the necessary macros, you can apply them to your Rust structures and enums to automatically generate the necessary parsing code. For example:

```rust, no_run
use cmake_parser_derive::CMake;
use cmake_parser::Token;

#[derive(CMake)]
struct MyCMakeData<'t> {
    output: Token<'t>,
    depends: Option<Vec<Token<'t>>>
}
```

For more information on how to use this library, please see the documentation for the individual derive macros.

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
cmake-parser = "0.1"
```
