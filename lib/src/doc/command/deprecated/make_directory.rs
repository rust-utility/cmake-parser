use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Creates the specified directory.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/make_directory.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct MakeDirectory<'t> {
    pub dir: Token<'t>,
}

impl<'t> ToCommandScope for MakeDirectory<'t> {
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
    fn make_directory() {
        let src = include_bytes!("../../../../../fixture/commands/deprecated/make_directory");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::MakeDirectory(Box::new(MakeDirectory {
                dir: token(b"dir1"),
            })),])
        )
    }
}
