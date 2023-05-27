use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add -D define flags to the compilation of source files.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_definitions.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct AddDefinitions<'t> {
    #[cmake(positional)]
    pub definitions: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for AddDefinitions<'t> {
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
    fn add_definitions() {
        let src = include_bytes!("../../../../../fixture/commands/project/add_definitions");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands().unwrap(),
            &[Command::AddDefinitions(Box::new(AddDefinitions {
                definitions: tokens([b"-DFOO", b"-DBAR"]).to_vec()
            }))]
        )
    }
}
