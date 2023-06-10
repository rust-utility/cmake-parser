use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Removes a binary directory.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_empty_binary_directory.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct CTestEmptyBinaryDirectory<'t> {
    #[cmake(positional)]
    pub directory: Token<'t>,
}

impl<'t> ToCommandScope for CTestEmptyBinaryDirectory<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::CTest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::token;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ctest_empty_binary_directory() {
        let src =
            include_bytes!("../../../../../fixture/commands/ctest/ctest_empty_binary_directory");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::CTestEmptyBinaryDirectory(Box::new(
                CTestEmptyBinaryDirectory {
                    directory: token(b"directory1"),
                }
            )),])
        )
    }
}
