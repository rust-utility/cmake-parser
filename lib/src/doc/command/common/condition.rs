use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Condition<'t> {
    pub conditions: Vec<Token<'t>>,
}
