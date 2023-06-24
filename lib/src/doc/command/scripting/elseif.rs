use cmake_parser_derive::CMake;

use crate::{
    command::common::Condition,
    doc::command_scope::{CommandScope, ToCommandScope},
};

/// Starts an elseif portion of an if block.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/elseif.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct ElseIf<'t> {
    pub condition: Option<Condition<'t>>,
}

impl<'t> ToCommandScope for ElseIf<'t> {
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
    fn elseif() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/elseif");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::ElseIf(Box::new(ElseIf { condition: None })),
                Command::ElseIf(Box::new(ElseIf {
                    condition: Some(Condition {
                        conditions: tokens_vec([b"VAR1"]),
                    })
                })),
            ])
        )
    }
}
