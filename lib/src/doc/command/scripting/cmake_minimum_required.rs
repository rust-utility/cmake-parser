use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Require a minimum version of cmake.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/cmake_minimum_required.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct CMakeMinimumRequired<'t> {
    pub version: Token<'t>,
}

impl<'t> ToCommandScope for CMakeMinimumRequired<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cmake_minimum_required() {
        let src =
            include_bytes!("../../../../../fixture/commands/scripting/cmake_minimum_required");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::CMakeMinimumRequired(Box::new(
                CMakeMinimumRequired {
                    version: token(b"version1"),
                }
            )),])
        )
    }
}
