use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Start recording a function for later invocation as a command.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/function.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Function<'t> {
    pub name: Token<'t>,
    pub args: Option<Vec<Token<'t>>>,
}

impl<'t> ToCommandScope for Function<'t> {
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
    fn function() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/function");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::Function(Box::new(Function {
                    name: token(b"name1"),
                    args: None,
                }))),
                Ok(Command::Function(Box::new(Function {
                    name: token(b"name1"),
                    args: Some(tokens_vec([b"arg1", b"arg2"])),
                }))),
            ]
        )
    }
}
