use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Ends a list of commands in a foreach block.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/endforeach.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct EndForEach<'t> {
    pub loop_var: Option<Token<'t>>,
}

impl<'t> ToCommandScope for EndForEach<'t> {
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
    fn endforeach() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/endforeach");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::EndForEach(Box::new(EndForEach { loop_var: None })),
                Command::EndForEach(Box::new(EndForEach {
                    loop_var: Some(token(b"VAR1")),
                })),
            ])
        )
    }
}
