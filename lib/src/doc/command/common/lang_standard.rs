use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum LangStandard<'t> {
    #[cmake(rename = "C_STANDARD")]
    C(Token<'t>),
    #[cmake(rename = "CXX_STANDARD")]
    Cxx(Token<'t>),
    #[cmake(rename = "OBJC_STANDARD")]
    ObjC(Token<'t>),
    #[cmake(rename = "OBJCXX_STANDARD")]
    ObjCxx(Token<'t>),
    #[cmake(rename = "CUDA_STANDARD")]
    Cuda(Token<'t>),
}
