use cmake_parser_derive::CMake;

use crate::{
    command::common::Condition,
    doc::command_scope::{CommandScope, ToCommandScope},
};

/// Conditionally execute a group of commands.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/if.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct If<'t> {
    pub condition: Condition<'t>,
}

impl<'t> ToCommandScope for If<'t> {
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
    fn r#if() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/if");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![Ok(Command::If(Box::new(If {
                condition: Condition {
                    conditions: tokens_vec([b"VAR1"]),
                },
            }))),]
        )
    }
}
