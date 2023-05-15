use ::cmake_parser_derive::CMake2;

use crate::Token;

#[derive(CMake2, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(positional, pkg = "crate")]
pub struct CustomCommand<'t> {
    pub name: Token<'t>,
    pub args: Vec<Token<'t>>,
}
