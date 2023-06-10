use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Sets the specified variable to a string representing the platform and compiler settings.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/build_name.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct BuildName<'t> {
    #[cmake(positional)]
    pub variable: Token<'t>,
}

impl<'t> ToCommandScope for BuildName<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Deprecated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn build_name() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/build_name");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::BuildName(Box::new(BuildName {
                variable: token(b"variable1"),
            })),])
        )
    }
}
