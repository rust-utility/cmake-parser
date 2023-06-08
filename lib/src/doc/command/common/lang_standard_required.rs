use cmake_parser_derive::CMake;

use crate::Token;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum LangStandardRequired<'t> {
    #[cmake(rename = "C_STANDARD_REQUIRED")]
    C(Token<'t>),
    #[cmake(rename = "CXX_STANDARD_REQUIRED")]
    Cxx(Token<'t>),
    #[cmake(rename = "OBJC_STANDARD_REQUIRED")]
    ObjC(Token<'t>),
    #[cmake(rename = "OBJCXX_STANDARD_REQUIRED")]
    ObjCxx(Token<'t>),
    #[cmake(rename = "CUDA_STANDARD_REQUIRED")]
    Cuda(Token<'t>),
}
