use cmake_parser_derive::CMake;

use crate::doc::command_scope::{CommandScope, ToCommandScope};

/// Break from an enclosing foreach or while loop.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/break.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct Break;

impl ToCommandScope for Break {
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
    fn r#break() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/break");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(doc.commands(), Ok(vec![Command::Break(Box::new(Break)),]))
    }
}
