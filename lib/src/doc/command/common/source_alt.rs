use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum SourceAlt<'t> {
    #[cmake(transparent)]
    Sources(Vec<Token<'t>>),
    Source(Token<'t>),
}
