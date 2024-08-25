use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Load and run CMake code from a file or module.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/include.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct Include<'t> {
    #[cmake(positional)]
    pub file: Token<'t>,
    pub optional: bool,
    pub result_variable: Option<Token<'t>>,
    pub no_policy_scope: bool,
}

impl<'t> ToCommandScope for Include<'t> {
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
    fn include() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/include");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::Include(Box::new(Include {
                    file: token(b"file1"),
                    optional: false,
                    result_variable: None,
                    no_policy_scope: false,
                }))),
                Ok(Command::Include(Box::new(Include {
                    file: token(b"file1"),
                    optional: false,
                    result_variable: Some(token(b"var1")),
                    no_policy_scope: false,
                }))),
                Ok(Command::Include(Box::new(Include {
                    file: token(b"file1"),
                    optional: true,
                    result_variable: Some(token(b"var1")),
                    no_policy_scope: true,
                }))),
            ]
        )
    }
}
