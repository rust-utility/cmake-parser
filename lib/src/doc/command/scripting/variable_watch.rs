use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Watch the CMake variable for change.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/variable_watch.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct VariableWatch<'t> {
    pub variable: Token<'t>,
    pub command: Option<Token<'t>>,
}

impl<'t> ToCommandScope for VariableWatch<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn variable_watch() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/variable_watch");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::VariableWatch(Box::new(VariableWatch {
                    variable: token(b"var1"),
                    command: None,
                }))),
                Ok(Command::VariableWatch(Box::new(VariableWatch {
                    variable: token(b"var1"),
                    command: Some(token(b"cmd1")),
                }))),
            ]
        )
    }
}
