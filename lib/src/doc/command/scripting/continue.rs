use cmake_parser_derive::CMake;

use crate::doc::command_scope::{CommandScope, ToCommandScope};

/// Continue to the top of enclosing foreach or while loop.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/continue.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct Continue;

impl ToCommandScope for Continue {
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
    fn r#continue() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/continue");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::Continue(Box::new(Continue)),])
        )
    }
}
