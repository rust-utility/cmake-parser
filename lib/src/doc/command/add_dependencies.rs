use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add a dependency between top-level targets.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_dependencies.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct AddDependencies<'t> {
    pub target: Token<'t>,
    pub target_dependencies: Option<Vec<Token<'t>>>,
}

impl<'t> ToCommandScope for AddDependencies<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens;
    use crate::*;

    #[test]
    fn add_dependencies() {
        let src = include_bytes!("../../../../fixture/commands/add_dependencies");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands().unwrap(),
            &[
                Command::AddDependencies(Box::new(AddDependencies {
                    target: b"target1".into(),
                    target_dependencies: None,
                })),
                Command::AddDependencies(Box::new(AddDependencies {
                    target: tokens([b"target2"])[0].clone(),
                    target_dependencies: Some(tokens([b"target-dep1", b"target-dep2"]).to_vec()),
                }))
            ]
        )
    }
}
