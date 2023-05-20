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
    Normal(NormalExecutable<'t>),
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct NormalExecutable<'t> {
    win32: bool,
    macosx_bundle: bool,
    exclude_from_all: bool,
    sources: Option<Vec<Token<'t>>>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens;
    use crate::*;

    #[test]
    fn add_compile_definitions() {
        let src = include_bytes!("../../../../fixture/commands/add_dependencies");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        /*
        assert_eq!(
            doc.commands().unwrap(),
            &[
                Command::AddExecutable(Box::new(AddExecutable {
                    target: b"target1".into(),
                    target_dependencies: None,
                })),
                Command::AddExecutable(Box::new(AddExecutable {
                    target: tokens([b"target2"])[0].clone(),
                    target_dependencies: Some(tokens([b"target-dep1", b"target-dep2"]).to_vec()),
                }))
            ]
        )
         */
    }
}
