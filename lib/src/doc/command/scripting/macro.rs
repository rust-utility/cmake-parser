use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Start recording a macro for later invocation as a command
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/macro.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Macro<'t> {
    pub name: Token<'t>,
    pub args: Option<Vec<Token<'t>>>,
}

impl<'t> ToCommandScope for Macro<'t> {
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
    fn r#macro() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/macro");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::Macro(Box::new(Macro {
                    name: token(b"name1"),
                    args: None,
                }))),
                Ok(Command::Macro(Box::new(Macro {
                    name: token(b"name1"),
                    args: Some(tokens_vec([b"arg1", b"arg2"])),
                }))),
            ]
        )
    }
}
