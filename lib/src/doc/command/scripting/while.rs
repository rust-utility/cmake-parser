use cmake_parser_derive::CMake;

use crate::{
    command::common::Condition,
    doc::command_scope::{CommandScope, ToCommandScope},
};

/// Evaluate a group of commands while a condition is true
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/while.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct While<'t> {
    pub condition: Condition<'t>,
}

impl<'t> ToCommandScope for While<'t> {
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
    fn r#while() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/while");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![Ok(Command::While(Box::new(While {
                condition: Condition {
                    conditions: tokens_vec([b"VAR1"]),
                }
            }))),]
        )
    }
}
