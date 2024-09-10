use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Set the given variable to the name of the computer.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/site_name.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct SiteName<'t> {
    #[cmake(positional)]
    pub variable: Token<'t>,
}

impl<'t> ToCommandScope for SiteName<'t> {
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
    fn site_name() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/site_name");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![Ok(Command::SiteName(Box::new(SiteName {
                variable: token(b"var1"),
            }))),]
        )
    }
}
