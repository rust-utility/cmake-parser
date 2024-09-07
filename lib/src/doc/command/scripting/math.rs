use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Evaluate a mathematical expression.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/math.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Math<'t> {
    #[cmake(rename = "EXPR", transparent)]
    pub variable: Token<'t>,
    pub expression: Token<'t>,
    #[cmake(transparent)]
    pub output_format: Option<Format>,
}

impl<'t> ToCommandScope for Math<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", list)]
pub enum Format {
    Hexadecimal,
    Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{quoted_token, token};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn math() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/math");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::Math(Box::new(Math {
                    variable: token(b"value"),
                    expression: quoted_token(b"100 % 10"),
                    output_format: None,
                }))),
                Ok(Command::Math(Box::new(Math {
                    variable: token(b"value"),
                    expression: quoted_token(b"100 * 0xA"),
                    output_format: Some(Format::Decimal),
                }))),
                Ok(Command::Math(Box::new(Math {
                    variable: token(b"value"),
                    expression: quoted_token(b"100 * 0xA"),
                    output_format: Some(Format::Hexadecimal),
                }))),
            ]
        )
    }
}
