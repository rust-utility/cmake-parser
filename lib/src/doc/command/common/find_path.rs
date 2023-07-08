use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum FindPath<'t> {
    #[cmake(transparent)]
    Env(Token<'t>),
    Path(Token<'t>),
}
