use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Specify libraries or flags to use when linking a given target and/or its dependents.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/target_link_libraries.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum TargetLinkLibraries<'t> {
    TargetAndOrDependents(TargetAndOrDependents<'t>),
    TargetAndOrDependentsLegacy(TargetAndOrDependentsLegacy<'t>),
    DependentsOnlyLegacy(DependentsOnlyLegacy<'t>),
    TargetAndDependents(TargetAndDependents<'t>),
}

impl<'t> ToCommandScope for TargetLinkLibraries<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TargetAndOrDependents<'t> {
    #[cmake(positional)]
    pub target: Token<'t>,
    pub libraries: Vec<Library<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum Library<'t> {
    Interface(Vec<Token<'t>>),
    Public(Vec<Token<'t>>),
    Private(Vec<Token<'t>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct TargetAndDependents<'t> {
    pub target: Token<'t>,
    pub libraries: Vec<Token<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TargetAndOrDependentsLegacy<'t> {
    #[cmake(positional)]
    pub target: Token<'t>,
    pub libraries: Vec<LinkLibrary<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum LinkLibrary<'t> {
    LinkPublic(Vec<Token<'t>>),
    LinkPrivate(Vec<Token<'t>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct DependentsOnlyLegacy<'t> {
    #[cmake(positional)]
    pub target: Token<'t>,
    pub link_interface_libraries: Vec<Token<'t>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn target_link_libraries() {
        let src = include_bytes!("../../../../../fixture/commands/project/target_link_libraries");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::TargetLinkLibraries(Box::new(TargetLinkLibraries::TargetAndOrDependents(
                    TargetAndOrDependents {
                        target: token(b"LibXml2"),
                        libraries: vec![Library::Private(tokens_vec([b"LibLZMA::LibLZMA"]))],
                    }
                ))),
                Command::TargetLinkLibraries(Box::new(TargetLinkLibraries::TargetAndDependents(
                    TargetAndDependents {
                        target: token(b"LibXml2Mod"),
                        libraries: tokens_vec([b"LibXml2", b"Python::Python"]),
                    }
                ))),
                Command::TargetLinkLibraries(Box::new(
                    TargetLinkLibraries::TargetAndOrDependentsLegacy(TargetAndOrDependentsLegacy {
                        target: token(b"${PROJECT_NAME}"),
                        libraries: vec![LinkLibrary::LinkPrivate(tokens_vec([
                            b"GTest::GTest",
                            b"GTest::Main",
                        ]))],
                    })
                )),
                Command::TargetLinkLibraries(Box::new(TargetLinkLibraries::DependentsOnlyLegacy(
                    DependentsOnlyLegacy {
                        target: token(b"${PROJECT_NAME}"),
                        link_interface_libraries: tokens_vec([b"GTest::GTest", b"GTest::Main"]),
                    }
                ))),
            ])
        )
    }
}
