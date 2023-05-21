use ::cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(positional, pkg = "crate")]
pub struct CustomCommand<'t> {
    pub name: Token<'t>,
    pub args: Option<Vec<Token<'t>>>,
}
