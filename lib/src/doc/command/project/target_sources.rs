use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add sources to a target.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/target_sources.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TargetSources<'t> {
    #[cmake(positional)]
    pub target: Token<'t>,
    pub sources: Vec<Source<'t>>,
}

impl<'t> ToCommandScope for TargetSources<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent)]
pub enum Source<'t> {
    Interface(Vec<SourceItem<'t>>),
    Public(Vec<SourceItem<'t>>),
    Private(Vec<SourceItem<'t>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged, transparent)]
pub enum SourceItem<'t> {
    FileSet(FileSet<'t>),
    Item(Token<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct FileSet<'t> {
    file_set: Token<'t>,
    #[cmake(rename = "TYPE")]
    file_set_type: Option<Token<'t>>,
    base_dirs: Option<Vec<Token<'t>>>,
    files: Option<Vec<Token<'t>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{quoted_token, token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn target_sources() {
        let src = include_bytes!("../../../../../fixture/commands/project/target_sources");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::TargetSources(Box::new(TargetSources {
                    target: token(b"MyTarget"),
                    sources: vec![Source::Private(vec![SourceItem::Item(quoted_token(
                        b"$<$<CONFIG:Debug>:${CMAKE_CURRENT_SOURCE_DIR}/dbgsrc.cpp>"
                    ))])]
                })),
                Command::TargetSources(Box::new(TargetSources {
                    target: token(b"${targetNameTests}"),
                    sources: vec![
                        Source::Private(vec![
                            SourceItem::Item(token(b"${ADD_TEST_TARGET_MAINFILE}")),
                            SourceItem::Item(token(b"${ADD_TEST_TARGET_PRIVATEFILES}")),
                            SourceItem::Item(token(b"${ADD_TEST_TARGET_TESTFILES}")),
                        ]),
                        Source::Public(vec![
                            SourceItem::Item(token(b"${ADD_TEST_TARGET_PUBLICFILES}")),
                            SourceItem::FileSet(FileSet {
                                file_set: token(b"${targetNameTests}_cxx_modules"),
                                file_set_type: Some(token(b"CXX_MODULES")),
                                base_dirs: None,
                                files: Some(tokens_vec([b"${ADD_TEST_TARGET_MODULEFILES}"]))
                            }),
                        ]),
                        Source::Interface(vec![SourceItem::Item(token(
                            b"${ADD_TEST_TARGET_INTERFACEFILES}"
                        )),])
                    ]
                })),
            ])
        )
    }
}
