use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct CopyFile<'t> {
    #[cmake(positional)]
    pub file_name: Token<'t>,
    pub copy_file_error: Option<Token<'t>>,
}
