use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add link directories to a target.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/target_link_directories.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TargetLinkDirectories<'t> {
    #[cmake(positional)]
    pub target: Token<'t>,
    pub before: bool,
    pub directories: Vec<Directory<'t>>,
}

impl<'t> ToCommandScope for TargetLinkDirectories<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
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
    fn target_link_directories() {
        let src = include_bytes!("../../../../../fixture/commands/project/target_link_directories");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::TargetLinkDirectories(Box::new(TargetLinkDirectories {
                    target: token(b"LibXml2"),
                    before: true,
                    directories: vec![Directory::Private(tokens_vec([
                        b"SYSCONFDIR=\"${CMAKE_INSTALL_FULL_SYSCONFDIR}\""
                    ]))]
                })),
                Command::TargetLinkDirectories(Box::new(TargetLinkDirectories {
                    target: token(b"LibXml2"),
                    before: false,
                    directories: vec![
                        Directory::Interface(tokens_vec([b"LIBXML_STATIC"])),
                        Directory::Private(tokens_vec([b"qqq", b"bbb"]))
                    ]
                })),
            ])
        )
    }
}
