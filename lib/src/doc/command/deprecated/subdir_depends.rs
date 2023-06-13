use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Does nothing.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/subdir_depends.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct SubdirDepends<'t> {
    pub subdir: Token<'t>,
    pub dependencies: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for SubdirDepends<'t> {
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
    fn subdir_depends() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/subdir_depends");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::SubdirDepends(Box::new(SubdirDepends {
                subdir: token(b"subdir1"),
                dependencies: tokens_vec([b"dep1", b"dep2"]),
            })),])
        )
    }
}
