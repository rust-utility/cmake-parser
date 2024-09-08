use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Get a property.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/get_property.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct GetProperty<'t> {
    pub variable: Token<'t>,
    #[cmake(in_range)]
    pub scope: Scope<'t>,
    #[cmake(transparent)]
    pub property: Token<'t>,
    pub options: Option<Options>,
}

impl<'t> ToCommandScope for GetProperty<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent, list)]
pub enum Scope<'t> {
    Global,
    #[cmake(positional)]
    Directory(Option<Token<'t>>),
    Target(Token<'t>),
    Source(Source<'t>),
    Install(Token<'t>),
    Test(Token<'t>),
    Cache(Token<'t>),
    Variable,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub enum Options {
    Set,
    Defined,
    BriefDocs,
    FullDocs,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Source<'t> {
    pub source: Token<'t>,
    pub scope: Option<DirectoryScope<'t>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", transparent, list)]
pub enum DirectoryScope<'t> {
    Directory(Token<'t>),
    TargetDirectory(Token<'t>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_property() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/get_property");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Global,
                    property: token(b"property1"),
                    options: None,
                }))),
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Directory(Some(token(b"directory1"))),
                    property: token(b"property1"),
                    options: Some(Options::Set),
                }))),
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Directory(None),
                    property: token(b"property1"),
                    options: Some(Options::Set),
                }))),
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Target(token(b"target1")),
                    property: token(b"property1"),
                    options: Some(Options::Defined),
                }))),
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Source(Source {
                        source: token(b"source1"),
                        scope: None
                    }),
                    property: token(b"property1"),
                    options: Some(Options::BriefDocs),
                }))),
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Source(Source {
                        source: token(b"source1"),
                        scope: Some(DirectoryScope::Directory(token(b"directory1"))),
                    }),
                    property: token(b"property1"),
                    options: Some(Options::FullDocs),
                }))),
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Source(Source {
                        source: token(b"source1"),
                        scope: Some(DirectoryScope::TargetDirectory(token(b"target_directory1"))),
                    }),
                    property: token(b"property1"),
                    options: None,
                }))),
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Install(token(b"file1")),
                    property: token(b"property1"),
                    options: Some(Options::Set),
                }))),
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Test(token(b"test1")),
                    property: token(b"property1"),
                    options: Some(Options::Defined),
                }))),
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Cache(token(b"entry1")),
                    property: token(b"property1"),
                    options: Some(Options::BriefDocs),
                }))),
                Ok(Command::GetProperty(Box::new(GetProperty {
                    variable: token(b"variable1"),
                    scope: Scope::Variable,
                    property: token(b"property1"),
                    options: Some(Options::FullDocs),
                }))),
            ]
        )
    }
}
