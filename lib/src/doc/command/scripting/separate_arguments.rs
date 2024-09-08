use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Parse command-line arguments into a semicolon-separated list.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/separate_arguments.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SeparateArguments<'t> {
    pub variable: Token<'t>,
    pub mode: Mode,
    #[cmake(transparent)]
    pub program: Option<Program>,
    pub args: Token<'t>,
}

impl<'t> ToCommandScope for SeparateArguments<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum Mode {
    UnixCommand,
    WindowsCommand,
    NativeCommand,
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Program {
    pub separate_args: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use doc::cmake_parse::tests::quoted_token;
    use pretty_assertions::assert_eq;

    #[test]
    fn separate_arguments() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/separate_arguments");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::SeparateArguments(Box::new(SeparateArguments {
                    variable: token(b"var1"),
                    mode: Mode::UnixCommand,
                    program: None,
                    args: quoted_token(b"cc -c main.c"),
                }))),
                Ok(Command::SeparateArguments(Box::new(SeparateArguments {
                    variable: token(b"var1"),
                    mode: Mode::WindowsCommand,
                    program: Some(Program {
                        separate_args: false,
                    }),
                    args: quoted_token(b"cc -c main.c"),
                }))),
                Ok(Command::SeparateArguments(Box::new(SeparateArguments {
                    variable: token(b"var1"),
                    mode: Mode::NativeCommand,
                    program: Some(Program {
                        separate_args: true,
                    }),
                    args: quoted_token(b"cc -c main.c"),
                }))),
            ]
        )
    }
}
