use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Define a grouping for source files in IDE project generation.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/source_group.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum SourceGroup<'t> {
    #[cmake(transparent)]
    Tree(TreeSourceGroup<'t>),
    Folder(FolderSourceGroup<'t>),
}

impl<'t> ToCommandScope for SourceGroup<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct TreeSourceGroup<'t> {
    #[cmake(positional)]
    pub root: Token<'t>,
    pub prefix: Option<Token<'t>>,
    pub files: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "regular_expression")]
pub struct FolderSourceGroup<'t> {
    #[cmake(positional)]
    pub group_name: Token<'t>,
    pub files: Option<Vec<Token<'t>>>,
    pub regular_expression: Option<Token<'t>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn source_group() {
        let src = include_bytes!("../../../../../fixture/commands/project/source_group");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::SourceGroup(Box::new(SourceGroup::Folder(FolderSourceGroup {
                    group_name: token(b"grp1"),
                    files: None,
                    regular_expression: Some(token(b"regex1")),
                }))),
                Command::SourceGroup(Box::new(SourceGroup::Folder(FolderSourceGroup {
                    group_name: token(b"grp2"),
                    files: None,
                    regular_expression: Some(token(b"regex2")),
                }))),
                Command::SourceGroup(Box::new(SourceGroup::Folder(FolderSourceGroup {
                    group_name: token(b"grp3"),
                    files: None,
                    regular_expression: None,
                }))),
                Command::SourceGroup(Box::new(SourceGroup::Folder(FolderSourceGroup {
                    group_name: token(b"grp4"),
                    files: Some(tokens_vec([b"file1", b"file2"])),
                    regular_expression: None,
                }))),
                Command::SourceGroup(Box::new(SourceGroup::Folder(FolderSourceGroup {
                    group_name: token(b"grp5"),
                    files: Some(tokens_vec([b"file3", b"file4"])),
                    regular_expression: Some(token(b"regex3")),
                }))),
                Command::SourceGroup(Box::new(SourceGroup::Tree(TreeSourceGroup {
                    root: token(b"grp6"),
                    prefix: None,
                    files: None,
                }))),
                Command::SourceGroup(Box::new(SourceGroup::Tree(TreeSourceGroup {
                    root: token(b"grp7"),
                    prefix: Some(token(b"prefix1")),
                    files: None,
                }))),
                Command::SourceGroup(Box::new(SourceGroup::Tree(TreeSourceGroup {
                    root: token(b"grp8"),
                    prefix: None,
                    files: Some(tokens_vec([b"file5", b"file6"])),
                }))),
                Command::SourceGroup(Box::new(SourceGroup::Tree(TreeSourceGroup {
                    root: token(b"grp9"),
                    prefix: Some(token(b"prefix2")),
                    files: Some(tokens_vec([b"file7", b"file8"])),
                }))),
            ])
        )
    }
}
