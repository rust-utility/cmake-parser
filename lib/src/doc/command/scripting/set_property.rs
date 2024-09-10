use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Set a named property in a given scope.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/set_property.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "values")]
pub struct SetProperty<'t> {
    pub scope: Scope<'t>,
    pub append: bool,
    pub append_string: bool,
    pub property: Token<'t>,
    #[cmake(rename = "")]
    pub values: Option<Vec<Token<'t>>>,
}

impl<'t> ToCommandScope for SetProperty<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent, complete)]
pub enum Scope<'t> {
    Global,
    #[cmake(positional)]
    Directory(Option<Token<'t>>),
    #[cmake(positional)]
    Target(Option<Vec<Token<'t>>>),
    Source(Source<'t>),
    #[cmake(positional)]
    Install(Option<Vec<Token<'t>>>),
    #[cmake(positional)]
    Test(Option<Vec<Token<'t>>>),
    #[cmake(positional)]
    Cache(Option<Vec<Token<'t>>>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", default = "source", allow_empty)]
pub struct Source<'t> {
    #[cmake(rename = "")]
    pub source: Option<Vec<Token<'t>>>,
    pub directory: Option<Vec<Token<'t>>>,
    pub target_directory: Option<Vec<Token<'t>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn set_property() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/set_property");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Global,
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Global,
                    append: true,
                    append_string: true,
                    property: token(b"property1"),
                    values: Some(tokens_vec([b"value1", b"value2"])),
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Directory(None),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Directory(Some(token(b"dir1"))),
                    append: true,
                    append_string: true,
                    property: token(b"property1"),
                    values: Some(tokens_vec([b"value1", b"value2"])),
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Target(None),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Target(Some(tokens_vec([b"target1", b"target2"]))),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Source(Source {
                        source: None,
                        directory: None,
                        target_directory: None,
                    }),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Source(Source {
                        source: Some(tokens_vec([b"src1", b"src2"])),
                        directory: None,
                        target_directory: None,
                    }),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Source(Source {
                        source: None,
                        directory: Some(tokens_vec([b"dir1", b"dir2"])),
                        target_directory: None,
                    }),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Source(Source {
                        source: None,
                        directory: None,
                        target_directory: Some(tokens_vec([b"tdir1", b"tdir2"])),
                    }),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Source(Source {
                        source: None,
                        directory: Some(tokens_vec([b"dir1", b"dir2"])),
                        target_directory: Some(tokens_vec([b"tdir1", b"tdir2"])),
                    }),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Source(Source {
                        source: Some(tokens_vec([b"src1", b"src2"])),
                        directory: Some(tokens_vec([b"dir1", b"dir2"])),
                        target_directory: Some(tokens_vec([b"tdir1", b"tdir2"])),
                    }),
                    append: true,
                    append_string: true,
                    property: token(b"property1"),
                    values: Some(tokens_vec([b"value1", b"value2"])),
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Install(None),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Install(Some(tokens_vec([b"install1", b"install2"]))),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Test(None),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Test(Some(tokens_vec([b"test1", b"test2"]))),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Cache(None),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
                Ok(Command::SetProperty(Box::new(SetProperty {
                    scope: Scope::Cache(Some(tokens_vec([b"entry1", b"entry2"]))),
                    append: false,
                    append_string: false,
                    property: token(b"property1"),
                    values: None,
                }))),
            ]
        )
    }
}
