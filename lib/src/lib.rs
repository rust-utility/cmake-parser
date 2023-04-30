#![doc = include_str!("../README.md")]

mod doc;
mod parser;

pub use doc::{command, Command, Doc, Token};
pub use parser::{parse_cmakelists, CMakeListsParseError, CMakeListsTokens};
