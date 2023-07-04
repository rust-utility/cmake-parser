use cmake_parser_derive::CMake;

use crate::command::common::{FileMatchKind, Permission};

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", match_fields)]
pub struct FileMatch<'t> {
    pub kind: Option<FileMatchKind<'t>>,
    pub exclude: bool,
    pub permissions: Option<Vec<Permission>>,
}
