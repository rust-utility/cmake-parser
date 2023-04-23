#![doc = include_str!("../README.md")]

mod doc;
mod parser;

pub use doc::{Command, Doc, TextNode, Utf8Doc, Utf8TextNode};
pub use parser::{parse_cmakelists, CMakeListsParseError, CMakeListsTokens};
