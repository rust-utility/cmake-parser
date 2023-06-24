use cmake_parser_derive::CMake;

use crate::{
    command::common::Condition,
    doc::command_scope::{CommandScope, ToCommandScope},
};

/// Ends a list of commands in an if block.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/endif.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct EndIf<'t> {
    pub condition: Option<Condition<'t>>,
}

impl<'t> ToCommandScope for EndIf<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens_vec;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn endif() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/endif");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::EndIf(Box::new(EndIf { condition: None })),
                Command::EndIf(Box::new(EndIf {
                    condition: Some(Condition {
                        conditions: tokens_vec([b"VAR1"]),
                    })
                })),
            ])
        )
    }
}
