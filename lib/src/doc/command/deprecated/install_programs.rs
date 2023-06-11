use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Specifies rules for installing programs for a project.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/install_programs.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum InstallPrograms<'t> {
    RegEx(InstallProgramsRegEx<'t>),
    Files(InstallProgramsFiles<'t>),
    FilesAlt(InstallProgramsFilesAlt<'t>),
}

impl<'t> ToCommandScope for InstallPrograms<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Deprecated
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct InstallProgramsFiles<'t> {
    #[cmake(positional)]
    pub dir: Token<'t>,
    pub files: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct InstallProgramsFilesAlt<'t> {
    pub dir: Token<'t>,
    pub files: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional, complete)]
pub struct InstallProgramsRegEx<'t> {
    pub dir: Token<'t>,
    pub regex: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn install_programs() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/install_programs");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::InstallPrograms(Box::new(InstallPrograms::Files(InstallProgramsFiles {
                    dir: token(b"dir1"),
                    files: tokens_vec([b"file1", b"file2"]),
                }))),
                Command::InstallPrograms(Box::new(InstallPrograms::RegEx(InstallProgramsRegEx {
                    dir: token(b"dir1"),
                    regex: token(b"regex1"),
                }))),
                Command::InstallPrograms(Box::new(InstallPrograms::FilesAlt(
                    InstallProgramsFilesAlt {
                        dir: token(b"dir1"),
                        files: tokens_vec([b"file1", b"file2"]),
                    }
                ))),
            ])
        )
    }
}
