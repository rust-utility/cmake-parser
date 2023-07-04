use cmake_parser_derive::CMake;

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum Permission {
    OwnerRead,
    OwnerWrite,
    OwnerExecute,
    GroupRead,
    GroupWrite,
    GroupExecute,
    WorldRead,
    WorldWrite,
    WorldExecute,
    #[cmake(rename = "SETUID")]
    SetUID,
    #[cmake(rename = "SETGID")]
    SetGID,
}
