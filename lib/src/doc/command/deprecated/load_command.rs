use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Load a command into a running CMake.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/load_command.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct LoadCommand<'t> {
    pub command_name: Token<'t>,
    pub locations: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for LoadCommand<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Deprecated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn load_command() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/load_command");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::LoadCommand(Box::new(LoadCommand {
                command_name: token(b"command_name1"),
                locations: tokens_vec([b"loc1", b"loc2"]),
            })),])
        )
    }
}
