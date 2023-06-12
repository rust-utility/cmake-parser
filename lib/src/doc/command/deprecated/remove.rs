use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Removes value from the variable.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/remove.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct Remove<'t> {
    pub var: Token<'t>,
    pub values: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for Remove<'t> {
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
    fn remove() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/remove");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::Remove(Box::new(Remove {
                var: token(b"var1"),
                values: tokens_vec([b"value1", b"value2"]),
            })),])
        )
    }
}
