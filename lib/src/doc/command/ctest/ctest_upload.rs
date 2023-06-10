use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Upload files to a dashboard server as a Dashboard Client.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_upload.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate")]
pub struct CTestUpload<'t> {
    pub files: Vec<Token<'t>>,
    pub capture_cmake_error: Option<Token<'t>>,
    pub quiet: bool,
}

impl<'t> ToCommandScope for CTestUpload<'t> {
    fn to_command_scope(&self) -> CommandScope {
        CommandScope::CTest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::cmake_parse::tests::{token, tokens_vec};
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ctest_upload() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_upload");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![Command::CTestUpload(Box::new(CTestUpload {
                files: tokens_vec([b"file1", b"file2"]),
                capture_cmake_error: Some(token(b"capture_cmake_error1")),
                quiet: true,
            })),])
        )
    }
}
