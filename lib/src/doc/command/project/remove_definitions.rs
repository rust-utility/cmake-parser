use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Remove -D define flags added by add_definitions().
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/remove_definitions.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct RemoveDefinitions<'t> {
    #[cmake(positional)]
    pub definitions: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for RemoveDefinitions<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Project
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens_vec;
    use crate::*;

    #[test]
    fn remove_definitions() {
        let src = include_bytes!("../../../../../fixture/commands/project/remove_definitions");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::RemoveDefinitions(Box::new(
                RemoveDefinitions {
                    definitions: tokens_vec([b"-DFOO", b"-DBAR"])
                }
            ))])
        )
    }
}
