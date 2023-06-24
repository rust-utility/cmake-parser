use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Ends a list of commands in a macro block.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/endmacro.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct EndMacro<'t> {
    pub name: Option<Token<'t>>,
}

impl<'t> ToCommandScope for EndMacro<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn endmacro() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/endmacro");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::EndMacro(Box::new(EndMacro { name: None })),
                Command::EndMacro(Box::new(EndMacro {
                    name: Some(token(b"name")),
                })),
            ])
        )
    }
}
