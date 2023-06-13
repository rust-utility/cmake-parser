use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Assert satisfaction of an option's required variables.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/variable_requires.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct VariableRequires<'t> {
    pub test_variable: Token<'t>,
    pub result_variable: Token<'t>,
    pub required_variables: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for VariableRequires<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Deprecated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn variable_requires() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/variable_requires");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::VariableRequires(Box::new(
                VariableRequires {
                    test_variable: token(b"test_variable1"),
                    result_variable: token(b"result_variable1"),
                    required_variables: tokens_vec([b"required_variable1", b"required_variable2"]),
                }
            )),])
        )
    }
}
