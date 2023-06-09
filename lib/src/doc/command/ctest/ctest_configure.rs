use cmake_parser_derive::CMake;

use crate::{
    doc::command_scope::{CommandScope, ToCommandScope},
    Token,
};

/// Perform the CTest Configure Step as a Dashboard Client.
///
/// Reference: <https://cmake.org/cmake/help/v3.26/command/ctest_configure.html>
#[derive(CMake, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cmake(pkg = "crate", allow_empty)]
pub struct CTestConfigure<'t> {
    #[cmake(rename = "BUILD")]
    pub build_dir: Option<Token<'t>>,
    #[cmake(rename = "SOURCE")]
    pub source_dir: Option<Token<'t>>,
    pub options: Option<Vec<Token<'t>>>,
    pub append: bool,
    pub return_value: Option<Token<'t>>,
    pub quiet: bool,
    pub capture_cmake_error: Option<Token<'t>>,
}

impl<'t> ToCommandScope for CTestConfigure<'t> {
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
    fn ctest_configure() {
        let src = include_bytes!("../../../../../fixture/commands/ctest/ctest_configure");
        let cmakelists = parse_cmakelists(src).unwrap();
        let doc = Doc::from(cmakelists);
        assert_eq!(
            doc.commands(),
            Ok(vec![
                Command::CTestConfigure(Box::new(CTestConfigure {
                    build_dir: None,
                    source_dir: None,
                    options: None,
                    append: false,
                    return_value: None,
                    quiet: false,
                    capture_cmake_error: None
                })),
                Command::CTestConfigure(Box::new(CTestConfigure {
                    build_dir: Some(token(b"build1")),
                    source_dir: Some(token(b"source1")),
                    options: Some(tokens_vec([b"opt1", b"opt2"])),
                    append: true,
                    return_value: Some(token(b"return_value1")),
                    quiet: true,
                    capture_cmake_error: Some(token(b"capture_cmake_error1"))
                })),
            ])
        )
    }
}
