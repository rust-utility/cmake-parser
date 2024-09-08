use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Return from a file, directory or function.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/return.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Return<'t> {
    #[cmake(transparent)]
    pub propagate: Option<Vec<Token<'t>>>,
}

impl<'t> ToCommandScope for Return<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::Scripting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens_vec;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn r#return() {
        let src = include_bytes!("../../../../../fixture/commands/scripting/return");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.to_commands_iter().collect::<Vec<_>>(),
            vec![
                Ok(Command::Return(Box::new(Return { propagate: None }))),
                Ok(Command::Return(Box::new(Return {
                    propagate: Some(tokens_vec([b"var1", b"var2"])),
                }))),
            ]
        )
    }
}
