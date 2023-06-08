use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum LangExtensions<'t> {
    #[cmake(rename = "C_EXTENSIONS")]
    C(Token<'t>),
    #[cmake(rename = "CXX_EXTENSIONS")]
    Cxx(Token<'t>),
    #[cmake(rename = "OBJC_EXTENSIONS")]
    ObjC(Token<'t>),
    #[cmake(rename = "OBJCXX_EXTENSIONS")]
    ObjCxx(Token<'t>),
    #[cmake(rename = "CUDA_EXTENSIONS")]
    Cuda(Token<'t>),
}
