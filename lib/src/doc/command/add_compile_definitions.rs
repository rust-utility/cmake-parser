use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Add preprocessor definitions to the compilation of source files.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/add_compile_definitions.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct AddCompileDefinitions<'t> {
    #[cmake(positional)]
    pub compile_definitions: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for AddCompileDefinitions<'t> {
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
    fn add_compile_definitions() {
        let src = include_bytes!("../../../../fixture/commands/project/add_compile_definitions");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands().unwrap(),
            &[Command::AddCompileDefinitions(Box::new(
                AddCompileDefinitions {
                    compile_definitions: tokens([b"DEBUG_UNPLUGGED"]).to_vec(),
                }
            ))]
        )
    }
}
