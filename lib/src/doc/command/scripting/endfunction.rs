use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Ends a list of commands in a function block.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/endfunction.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct EndFunction<'t> {
    pub name: Option<Token<'t>>,
}

impl<'t> ToCommandScope for EndFunction<'t> {
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
    fn endfunction() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/endfunction");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::EndFunction(Box::new(EndFunction { name: None })),
                Command::EndFunction(Box::new(EndFunction {
                    name: Some(token(b"name")),
                })),
            ])
        )
    }
}
