use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum FileMatchKind<'t> {
    Pattern(Token<'t>),
    #[cmake(rename = "REGEX")]
    RegEx(Token<'t>),
}
