use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Keyword, Token,
};

/// Add an executable to the project using the specified source files.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_executable.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct AddExecutable<'t> {
    pub name: Token<'t>,
    pub executable: Executable<'t>,
}

impl<'t> ToCommandScope for AddExecutable<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[cmake(pkg = "crate", untagged)]
pub enum Executable<'t> {
    Alias(AliasExecutable<'t>),
    Imported(ImportedExecutable),
    Normal(NormalExecutable<'t>),
}

impl<'t> crate::CMakeParse<'t> for Executable<'t> {
    fn parse<'tv>(
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), crate::CommandParseError> {
        use crate::CMakeParse;
        Err(crate::CommandParseError::TokenRequired)
            .or_else(|_| CMakeParse::parse(tokens).map(|(res, tokens)| (Self::Alias(res), tokens)))
            .or_else(|_| {
                CMakeParse::parse(tokens).map(|(res, tokens)| (Self::Imported(res), tokens))
            })
            .or_else(|_| CMakeParse::parse(tokens).map(|(res, tokens)| (Self::Normal(res), tokens)))
    }
}

impl<'t> crate::CMakePositional<'t> for Executable<'t> {
    fn positional<'tv>(
        _: &'static [u8],
        tokens: &'tv [Token<'t>],
    ) -> Result<(Self, &'tv [Token<'t>]), crate::CommandParseError> {
        crate::CMakeParse::parse(tokens)
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct NormalExecutable<'t> {
    pub win32: bool,
    pub macosx_bundle: bool,
    pub exclude_from_all: bool,
    pub sources: Option<Vec<Token<'t>>>,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ImportedExecutable {
    imported: Keyword,
    pub global: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct AliasExecutable<'t> {
    alias: Keyword,
    pub target: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn add_compile_definitions() {
        let src = include_bytes!("../../../../fixture/commands/add_executable");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::AddExecutable(Box::new(AddExecutable {
                    name: b"MyProgram".into(),
                    executable: Executable::Normal(NormalExecutable {
                        win32: false,
                        macosx_bundle: false,
                        exclude_from_all: true,
                        sources: Some(vec![b"my_program.cpp".into(),],),
                    },),
                }),),
                Command::AddExecutable(Box::new(AddExecutable {
                    name: b"ClangFormat".into(),
                    executable: Executable::Imported(ImportedExecutable {
                        imported: Keyword,
                        global: false,
                    },),
                }),),
                Command::AddExecutable(Box::new(AddExecutable {
                    name: b"MyAliasedProgram".into(),
                    executable: Executable::Alias(AliasExecutable {
                        alias: Keyword,
                        target: b"MyProgram".into(),
                    },),
                }),),
            ],)
        );
    }
}
