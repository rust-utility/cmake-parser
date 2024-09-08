use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Provide a boolean option that the user can optionally select.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/option.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Option<'t> {
    pub variable: Token<'t>,
    pub help_text: Token<'t>,
    pub value: std::option::Option<Token<'t>>,
}

impl<'t> ToCommandScope for Option<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{quoted_token, token};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn option() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/option");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::Option(Box::new(Option {
                    variable: token(b"var1"),
                    help_text: quoted_token(b"enter var1"),
                    value: None,
                }))),
                Ok(Command::Option(Box::new(Option {
                    variable: token(b"var1"),
                    help_text: quoted_token(b"enter var1"),
                    value: Some(token(b"value1")),
                }))),
            ]
        )
    }
}
