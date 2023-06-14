use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Evaluate a group of commands with a dedicated variable and/or policy scope.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/block.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", allow_empty)]
pub struct Block<'t> {
    scope_for: Option<ScopeFor>,
    propagate: Option<Vec<Token<'t>>>,
}

impl<'t> ToCommandScope for Block<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct ScopeFor {
    policies: bool,
    variables: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens_vec;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn block() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/block");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::Block(Box::new(Block {
                    scope_for: None,
                    propagate: None,
                })),
                Command::Block(Box::new(Block {
                    scope_for: Some(ScopeFor {
                        policies: true,
                        variables: false,
                    }),
                    propagate: None,
                })),
                Command::Block(Box::new(Block {
                    scope_for: Some(ScopeFor {
                        policies: false,
                        variables: true,
                    }),
                    propagate: None,
                })),
                Command::Block(Box::new(Block {
                    scope_for: None,
                    propagate: Some(tokens_vec([b"var1", b"var2"])),
                })),
                Command::Block(Box::new(Block {
                    scope_for: Some(ScopeFor {
                        policies: true,
                        variables: true,
                    }),
                    propagate: Some(tokens_vec([b"var1", b"var2"])),
                })),
            ])
        )
    }
}
