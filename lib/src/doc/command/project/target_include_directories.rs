use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add include directories to a target.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/target_include_directories.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TargetIncludeDirectories<'t> {
    #[cmake(positional)]
    pub target: Token<'t>,
    pub system: bool,
    pub mode: Option<Mode>,
    pub directories: Option<Vec<Directory<'t>>>,
}

impl<'t> ToCommandScope for TargetIncludeDirectories<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum Mode {
    After,
    Before,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum Directory<'t> {
    Interface(Vec<Token<'t>>),
    Public(Vec<Token<'t>>),
    Private(Vec<Token<'t>>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn target_include_directories() {
        let src =
            include_bytes!("../../../../../fixture/commands/project/target_include_directories");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::TargetIncludeDirectories(Box::new(TargetIncludeDirectories {
                    target: token(b"name"),
                    system: false,
                    mode: None,
                    directories: None,
                })),
                Command::TargetIncludeDirectories(Box::new(TargetIncludeDirectories {
                    target: token(b"mylib"),
                    system: false,
                    mode: None,
                    directories: Some(vec![Directory::Public(tokens_vec([
                        b"$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mylib>",
                        b"$<INSTALL_INTERFACE:include/mylib>",
                    ]))]),
                })),
                Command::TargetIncludeDirectories(Box::new(TargetIncludeDirectories {
                    target: token(b"mylib"),
                    system: true,
                    mode: Some(Mode::Before),
                    directories: Some(vec![Directory::Public(tokens_vec([
                        b"$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mylib>",
                        b"$<INSTALL_INTERFACE:include/mylib>",
                    ]))]),
                })),
                Command::TargetIncludeDirectories(Box::new(TargetIncludeDirectories {
                    target: token(b"mylib"),
                    system: false,
                    mode: Some(Mode::After),
                    directories: Some(vec![
                        Directory::Interface(vec![token(
                            b"$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include/mylib>"
                        )]),
                        Directory::Private(vec![token(b"$<INSTALL_INTERFACE:include/mylib>")])
                    ]),
                })),
            ])
        )
    }
}
