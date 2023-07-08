use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, complete)]
pub enum Names<'t> {
    #[cmake(rename = "NAMES", transparent)]
    Multi(Vec<Token<'t>>),
    Single(Token<'t>),
}
