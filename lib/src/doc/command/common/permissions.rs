use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum Permissions<'t> {
    #[cmake(rename = "NO_SOURCE_PERMISSIONS")]
    NoSource,
    #[cmake(rename = "USE_SOURCE_PERMISSIONS")]
    UseSource,
    #[cmake(rename = "FILE_PERMISSIONS", transparent)]
    File(Vec<Token<'t>>),
}
