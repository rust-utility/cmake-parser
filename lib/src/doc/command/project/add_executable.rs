use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
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

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", untagged)]
pub enum Executable<'t> {
    #[cmake(transparent)]
    Alias(AliasExecutable<'t>),
    #[cmake(transparent)]
    Imported(ImportedExecutable),
    Normal(NormalExecutable<'t>),
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
    pub global: bool,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct AliasExecutable<'t> {
    pub target: Token<'t>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn add_executable() {
        let src = include_bytes!("../../../../../fixture/commands/project/add_executable");
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
                    executable: Executable::Imported(ImportedExecutable { global: false },),
                }),),
                Command::AddExecutable(Box::new(AddExecutable {
                    name: b"MyAliasedProgram".into(),
                    executable: Executable::Alias(AliasExecutable {
                        target: b"MyProgram".into(),
                    },),
                }),),
            ],)
        );
    }
}
