use cmake_parser_derive::CMake;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum Append {
    After,
    Before,
}
