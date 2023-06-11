use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Specifies rules for installing files for a project.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/install_files.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum InstallFiles<'t> {
    Files(InstallFilesAll<'t>),
    Extension(InstallFilesExtension<'t>),
    RegEx(InstallFilesRegEx<'t>),
}

impl<'t> ToCommandScope for InstallFiles<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Deprecated
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct InstallFilesAll<'t> {
    #[cmake(positional)]
    pub dir: Token<'t>,
    pub files: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct InstallFilesRegEx<'t> {
    pub dir: Token<'t>,
    pub regex: Token<'t>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct InstallFilesExtension<'t> {
    pub dir: Token<'t>,
    pub extension: Token<'t>,
    pub files: Vec<Token<'t>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn install_files() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/install_files");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::InstallFiles(Box::new(InstallFiles::Files(InstallFilesAll {
                    dir: token(b"dir1"),
                    files: tokens_vec([b"file1", b"file2"]),
                }))),
                Command::InstallFiles(Box::new(InstallFiles::RegEx(InstallFilesRegEx {
                    dir: token(b"dir1"),
                    regex: token(b"regex1"),
                }))),
                Command::InstallFiles(Box::new(InstallFiles::Extension(InstallFilesExtension {
                    dir: token(b"dir1"),
                    extension: token(b"extension1"),
                    files: tokens_vec([b"file1", b"file2"]),
                }))),
            ])
        )
    }
}
