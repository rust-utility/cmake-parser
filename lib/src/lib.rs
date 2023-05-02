#![doc = include_str!("../README.md")]

mod doc;
mod parser;

pub use cmake_parser_derive::CMake;
pub use doc::{
    command::{self, CMakeCommand, CommandParseError},
    declarations_by_keywords, Command, CommandScope, Doc, TextNodeDeclaration, ToCommandScope,
    Token, TokenDeclarations,
};
pub use parser::{parse_cmakelists, CMakeListsParseError, CMakeListsTokens};
