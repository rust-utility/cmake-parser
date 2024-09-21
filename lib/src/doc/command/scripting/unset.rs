use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Unset a variable, cache variable, or environment variable.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/unset.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Unset<'t> {
    pub variable: Token<'t>,
    pub scope: Option<Scope>,
}

impl<'t> ToCommandScope for Unset<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub enum Scope {
    Cache,
    ParentScope,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn unset() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/unset");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::Unset(Box::new(Unset {
                    variable: token(b"var1"),
                    scope: None,
                }))),
                Ok(Command::Unset(Box::new(Unset {
                    variable: token(b"var1"),
                    scope: Some(Scope::Cache),
                }))),
                Ok(Command::Unset(Box::new(Unset {
                    variable: token(b"var1"),
                    scope: Some(Scope::ParentScope),
                }))),
            ]
        )
    }
}
