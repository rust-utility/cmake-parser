use cmake_parser_derive::CMake;

use crate::doc::command_scope::{CommandScope, ToCommandScope};

/// Ends a list of commands in a block() and removes the scopes created by the block() command.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/endblock.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct EndBlock;

impl ToCommandScope for EndBlock {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn endblock() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/endblock");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::EndBlock(Box::new(EndBlock)),])
        )
    }
}
