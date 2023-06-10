use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Read all the CTestCustom.ctest or CTestCustom.cmake files from the given directory.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_read_custom_files.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", positional)]
pub struct CTestReadCustomFiles<'t> {
    pub directories: Vec<Token<'t>>,
}

impl<'t> ToCommandScope for CTestReadCustomFiles<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::CTest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::tokens_vec;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ctest_read_custom_files() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_read_custom_files");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::CTestReadCustomFiles(Box::new(
                CTestReadCustomFiles {
                    directories: tokens_vec([b"dir1", b"dir2", b"dir3"]),
                }
            )),])
        )
    }
}
