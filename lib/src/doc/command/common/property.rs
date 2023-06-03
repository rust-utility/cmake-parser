use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Property<'t> {
    pub prop: Token<'t>,
    pub value: Token<'t>,
}
