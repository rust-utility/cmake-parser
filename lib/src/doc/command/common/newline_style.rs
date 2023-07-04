use cmake_parser_derive::CMake;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum NewlineStyle {
    Unix,
    Dos,
    Win32,
    Lf,
    #[cmake(rename = "CRLF")]
    CrLf,
}
