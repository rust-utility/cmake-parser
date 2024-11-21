#![doc = include_str!("../README.md")]

mod doc;
pub mod parser;

use std::ops::Range;

pub use cmake_parser_derive::CMake;
pub use doc::{
    command::{self, CommandParseError},
    declarations_by_keywords, CMakeParse, CMakePositional, Command, CommandScope, Doc, Keyword,
    TextNodeDeclaration, ToCommandScope, Token, TokenDeclarations,
};
pub use parser::{parse_cmakelists, CMakeListsParseError, CMakeListsTokens, FileElement};

/// ```rust
/// # use cmake_parser::slice_subspan;
/// let parent = b"hello world";
/// let child = &parent[6..];
/// let other = b"other";
/// assert_eq!(Some(6..11), slice_subspan(parent, child));
/// assert_eq!(None, slice_subspan(parent, other));
/// ```
pub fn slice_subspan(parent: &[u8], child: &[u8]) -> Option<Range<usize>> {
    let range = child.as_ptr_range();
    let parent_range = parent.as_ptr_range();
    if parent_range.contains(&range.start) {
        let start = child.as_ptr() as usize - parent.as_ptr() as usize;
        Some(start..(start + child.len()).min(parent.len()))
    } else {
        None
    }
}
